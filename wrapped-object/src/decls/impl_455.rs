macro_rules! deps {
    () => {
        Error!();
        Bytes!();
        AttributesSubsubsection!();
        Result!();
        AttributesSubsubsectionIterator!();
        U32Bytes!();
        FileHeader!();
        Endian!();
    };
}

macro_rules! impl_455 {
    () => {
        deps!();
        impl < 'data , Elf : FileHeader > AttributesSubsubsectionIterator < 'data , Elf > { # [doc = " Return the next sub-subsection."] pub fn next (& mut self) -> Result < Option < AttributesSubsubsection < 'data > > > { if self . data . is_empty () { return Ok (None) ; } let result = self . parse () . map (Some) ; if result . is_err () { self . data = Bytes (& []) ; } result } fn parse (& mut self) -> Result < AttributesSubsubsection < 'data > > { let mut data = self . data ; let tag = * data . read :: < u8 > () . read_error ("ELF attributes subsection is too short") ? ; let length = data . read :: < endian :: U32Bytes < Elf :: Endian > > () . read_error ("ELF attributes subsection is too short") ? . get (self . endian) ; let mut data = self . data . read_bytes (length as usize) . read_error ("Invalid ELF attributes sub-subsection length") ? ; data . skip (1 + 4) . read_error ("Invalid ELF attributes sub-subsection length") ? ; let indices = if tag == elf :: Tag_Section || tag == elf :: Tag_Symbol { data . read_string () . map (Bytes) . read_error ("Missing ELF attributes sub-subsection indices") ? } else if tag == elf :: Tag_File { Bytes (& []) } else { return Err (Error ("Unimplemented ELF attributes sub-subsection tag")) ; } ; Ok (AttributesSubsubsection { tag , length , indices , data , }) } }
    };
}

impl_455!()