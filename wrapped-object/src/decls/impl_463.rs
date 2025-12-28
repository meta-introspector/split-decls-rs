macro_rules! deps {
    () => {
        Result!();
        AttributeReader!();
    };
}

macro_rules! impl_463 {
    () => {
        deps!();
        impl < 'data > AttributeReader < 'data > { # [doc = " Parse a tag."] pub fn read_tag (& mut self) -> Result < Option < u64 > > { if self . data . is_empty () { return Ok (None) ; } let err = "Invalid ELF attribute tag" ; self . data . read_uleb128 () . read_error (err) . map (Some) } # [doc = " Parse an integer value."] pub fn read_integer (& mut self) -> Result < u64 > { let err = "Invalid ELF attribute integer value" ; self . data . read_uleb128 () . read_error (err) } # [doc = " Parse a string value."] pub fn read_string (& mut self) -> Result < & 'data [u8] > { let err = "Invalid ELF attribute string value" ; self . data . read_string () . read_error (err) } }
    };
}

impl_463!();