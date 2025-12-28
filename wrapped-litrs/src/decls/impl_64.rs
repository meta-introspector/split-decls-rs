macro_rules! deps {
    () => {
        ParseError!();
        CharLit!();
        Buffer!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < B : Buffer > CharLit < B > { # [doc = " Parses the input as a character literal. Returns an error if the input"] # [doc = " is invalid or represents a different kind of literal."] pub fn parse (input : B) -> Result < Self , ParseError > { match first_byte_or_empty (& input) ? { b'\'' => { let (value , start_suffix) = parse_impl (& input) ? ; Ok (Self { raw : input , value , start_suffix }) } _ => Err (perr (0 , DoesNotStartWithQuote)) , } } # [doc = " Returns the character value that this literal represents."] pub fn value (& self) -> char { self . value } # [doc = " The optional suffix. Returns `\"\"` if the suffix is empty/does not exist."] pub fn suffix (& self) -> & str { & (* self . raw) [self . start_suffix ..] } # [doc = " Returns the raw input that was passed to `parse`."] pub fn raw_input (& self) -> & str { & self . raw } # [doc = " Returns the raw input that was passed to `parse`, potentially owned."] pub fn into_raw_input (self) -> B { self . raw } }
    };
}

impl_64!();