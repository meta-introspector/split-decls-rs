macro_rules! deps {
    () => {
        VerdauxIterator!();
        FileHeader!();
        Bytes!();
        Verdaux!();
        Result!();
        Endian!();
    };
}

macro_rules! impl_435 {
    () => {
        deps!();
        impl < 'data , Elf : FileHeader > VerdauxIterator < 'data , Elf > { pub (super) fn new (endian : Elf :: Endian , data : & 'data [u8] , count : u16) -> Self { VerdauxIterator { endian , data : Bytes (data) , count , } } # [doc = " Return the next `Verdaux` entry."] pub fn next (& mut self) -> Result < Option < & 'data elf :: Verdaux < Elf :: Endian > > > { if self . count == 0 { return Ok (None) ; } let result = self . parse () . map (Some) ; if result . is_err () { self . count = 0 ; } else { self . count -= 1 ; } result } fn parse (& mut self) -> Result < & 'data elf :: Verdaux < Elf :: Endian > > { let verdaux = self . data . read_at :: < elf :: Verdaux < _ > > (0) . read_error ("ELF verdaux is too short") ? ; self . data . skip (verdaux . vda_next . get (self . endian) as usize) . read_error ("Invalid ELF vda_next") ? ; Ok (verdaux) } }
    };
}

impl_435!();