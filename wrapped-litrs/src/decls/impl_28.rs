macro_rules! deps {
    () => {
        Buffer!();
        ByteLit!();
        ParseError!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < B : Buffer > ByteLit < B > { # [doc = " Parses the input as a byte literal. Returns an error if the input is"] # [doc = " invalid or represents a different kind of literal."] pub fn parse (input : B) -> Result < Self , ParseError > { if input . is_empty () { return Err (perr (None , Empty)) ; } if ! input . starts_with ("b'") { return Err (perr (None , InvalidByteLiteralStart)) ; } let (value , start_suffix) = parse_impl (& input) ? ; Ok (Self { raw : input , value , start_suffix }) } # [doc = " Returns the byte value that this literal represents."] pub fn value (& self) -> u8 { self . value } # [doc = " The optional suffix. Returns `\"\"` if the suffix is empty/does not exist."] pub fn suffix (& self) -> & str { & (* self . raw) [self . start_suffix ..] } # [doc = " Returns the raw input that was passed to `parse`."] pub fn raw_input (& self) -> & str { & self . raw } # [doc = " Returns the raw input that was passed to `parse`, potentially owned."] pub fn into_raw_input (self) -> B { self . raw } }
    };
}

impl_28!();