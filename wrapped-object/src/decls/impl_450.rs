macro_rules! deps {
    () => {
        AttributesSubsectionIterator!();
        U32Bytes!();
        Endian!();
        Result!();
        Bytes!();
        AttributesSubsection!();
        FileHeader!();
    };
}

macro_rules! impl_450 {
    () => {
        deps!();
        impl < 'data , Elf : FileHeader > AttributesSubsectionIterator < 'data , Elf > { # [doc = " Return the next subsection."] pub fn next (& mut self) -> Result < Option < AttributesSubsection < 'data , Elf > > > { if self . data . is_empty () { return Ok (None) ; } let result = self . parse () . map (Some) ; if result . is_err () { self . data = Bytes (& []) ; } result } fn parse (& mut self) -> Result < AttributesSubsection < 'data , Elf > > { let mut data = self . data ; let length = data . read :: < endian :: U32Bytes < Elf :: Endian > > () . read_error ("ELF attributes section is too short") ? . get (self . endian) ; let mut data = self . data . read_bytes (length as usize) . read_error ("Invalid ELF attributes subsection length") ? ; data . skip (4) . read_error ("Invalid ELF attributes subsection length") ? ; let vendor = data . read_string () . read_error ("Invalid ELF attributes vendor") ? ; Ok (AttributesSubsection { endian : self . endian , length , vendor , data , }) } }
    };
}

impl_450!()