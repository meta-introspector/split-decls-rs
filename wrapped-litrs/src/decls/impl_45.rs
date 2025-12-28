macro_rules! deps {
    () => {
        ParseError!();
        Buffer!();
        ByteStringLit!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < B : Buffer > ByteStringLit < B > { # [doc = " Parses the input as a (raw) byte string literal. Returns an error if the"] # [doc = " input is invalid or represents a different kind of literal."] pub fn parse (input : B) -> Result < Self , ParseError > { if input . is_empty () { return Err (perr (None , Empty)) ; } if ! input . starts_with (r#"b""#) && ! input . starts_with ("br") { return Err (perr (None , InvalidByteStringLiteralStart)) ; } let (value , num_hashes , start_suffix) = parse_impl (& input) ? ; Ok (Self { raw : input , value , num_hashes , start_suffix }) } # [doc = " Returns the string value this literal represents (where all escapes have"] # [doc = " been turned into their respective values)."] pub fn value (& self) -> & [u8] { self . value . as_deref () . unwrap_or (& self . raw . as_bytes () [self . inner_range ()]) } # [doc = " Like `value` but returns a potentially owned version of the value."] # [doc = ""] # [doc = " The return value is either `Vec<u8>` if `B = String`, or"] # [doc = " `Cow<'a, [u8]>` if `B = &'a str`."] pub fn into_value (self) -> B :: ByteCow { let inner_range = self . inner_range () ; let Self { raw , value , .. } = self ; value . map (B :: ByteCow :: from) . unwrap_or_else (| | raw . cut (inner_range) . into_byte_cow ()) } # [doc = " The optional suffix. Returns `\"\"` if the suffix is empty/does not exist."] pub fn suffix (& self) -> & str { & (* self . raw) [self . start_suffix ..] } # [doc = " Returns whether this literal is a raw string literal (starting with"] # [doc = " `r`)."] pub fn is_raw_byte_string (& self) -> bool { self . num_hashes . is_some () } # [doc = " Returns the raw input that was passed to `parse`."] pub fn raw_input (& self) -> & str { & self . raw } # [doc = " Returns the raw input that was passed to `parse`, potentially owned."] pub fn into_raw_input (self) -> B { self . raw } # [doc = " The range within `self.raw` that excludes the quotes and potential `r#`."] fn inner_range (& self) -> Range < usize > { match self . num_hashes { None => 2 .. self . start_suffix - 1 , Some (n) => 2 + n as usize + 1 .. self . start_suffix - n as usize - 1 , } } }
    };
}

impl_45!()