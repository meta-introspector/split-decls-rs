macro_rules! deps {
    () => {
        Endian!();
        FileHeader!();
        Bytes!();
        VerdefIterator!();
        Result!();
        VerdauxIterator!();
        Verdef!();
    };
}

macro_rules! impl_432 {
    () => {
        deps!();
        impl < 'data , Elf : FileHeader > VerdefIterator < 'data , Elf > { pub (super) fn new (endian : Elf :: Endian , data : & 'data [u8]) -> Self { VerdefIterator { endian , data : Bytes (data) , } } # [doc = " Return the next `Verdef` entry."] pub fn next (& mut self ,) -> Result < Option < (& 'data elf :: Verdef < Elf :: Endian > , VerdauxIterator < 'data , Elf >) > > { if self . data . is_empty () { return Ok (None) ; } let result = self . parse () . map (Some) ; if result . is_err () { self . data = Bytes (& []) ; } result } fn parse (& mut self) -> Result < (& 'data elf :: Verdef < Elf :: Endian > , VerdauxIterator < 'data , Elf >) > { let verdef = self . data . read_at :: < elf :: Verdef < _ > > (0) . read_error ("ELF verdef is too short") ? ; let mut verdaux_data = self . data ; verdaux_data . skip (verdef . vd_aux . get (self . endian) as usize) . read_error ("Invalid ELF vd_aux") ? ; let verdaux = VerdauxIterator :: new (self . endian , verdaux_data . 0 , verdef . vd_cnt . get (self . endian)) ; let next = verdef . vd_next . get (self . endian) ; if next != 0 { self . data . skip (next as usize) . read_error ("Invalid ELF vd_next") ? ; } else { self . data = Bytes (& []) ; } Ok ((verdef , verdaux)) } }
    };
}

impl_432!();