macro_rules! deps {
    () => {
        VerneedIterator!();
        Verneed!();
        VernauxIterator!();
        Endian!();
        Result!();
        FileHeader!();
        Bytes!();
    };
}

macro_rules! impl_438 {
    () => {
        deps!();
        impl < 'data , Elf : FileHeader > VerneedIterator < 'data , Elf > { pub (super) fn new (endian : Elf :: Endian , data : & 'data [u8]) -> Self { VerneedIterator { endian , data : Bytes (data) , } } # [doc = " Return the next `Verneed` entry."] pub fn next (& mut self ,) -> Result < Option < (& 'data elf :: Verneed < Elf :: Endian > , VernauxIterator < 'data , Elf > ,) > , > { if self . data . is_empty () { return Ok (None) ; } let result = self . parse () . map (Some) ; if result . is_err () { self . data = Bytes (& []) ; } result } fn parse (& mut self ,) -> Result < (& 'data elf :: Verneed < Elf :: Endian > , VernauxIterator < 'data , Elf > ,) > { let verneed = self . data . read_at :: < elf :: Verneed < _ > > (0) . read_error ("ELF verneed is too short") ? ; let mut vernaux_data = self . data ; vernaux_data . skip (verneed . vn_aux . get (self . endian) as usize) . read_error ("Invalid ELF vn_aux") ? ; let vernaux = VernauxIterator :: new (self . endian , vernaux_data . 0 , verneed . vn_cnt . get (self . endian)) ; let next = verneed . vn_next . get (self . endian) ; if next != 0 { self . data . skip (next as usize) . read_error ("Invalid ELF vn_next") ? ; } else { self . data = Bytes (& []) ; } Ok ((verneed , vernaux)) } }
    };
}

impl_438!()