macro_rules! deps {
    () => {
        Result!();
        AttributeIndexIterator!();
        Bytes!();
    };
}

macro_rules! impl_460 {
    () => {
        deps!();
        impl < 'data > AttributeIndexIterator < 'data > { # [doc = " Parse the next index."] pub fn next (& mut self) -> Result < Option < u32 > > { if self . data . is_empty () { return Ok (None) ; } let result = self . parse () . map (Some) ; if result . is_err () { self . data = Bytes (& []) ; } result } fn parse (& mut self) -> Result < u32 > { let err = "Invalid ELF attribute index" ; self . data . read_uleb128 () . read_error (err) ? . try_into () . map_err (| _ | ()) . read_error (err) } }
    };
}

impl_460!();