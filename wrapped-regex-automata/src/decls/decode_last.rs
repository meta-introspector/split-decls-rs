macro_rules! decode_last {
    () => {
        # [doc = " Decodes the last UTF-8 encoded codepoint from the given byte slice."] # [doc = ""] # [doc = " If no valid encoding of a codepoint exists at the end of the given byte"] # [doc = " slice, then the last byte is returned instead."] # [doc = ""] # [doc = " This returns `None` if and only if `bytes` is empty."] # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn decode_last (bytes : & [u8]) -> Option < Result < char , u8 > > { if bytes . is_empty () { return None ; } let mut start = bytes . len () - 1 ; let limit = bytes . len () . saturating_sub (4) ; while start > limit && ! is_leading_or_invalid_byte (bytes [start]) { start -= 1 ; } match decode (& bytes [start ..]) { None => None , Some (Ok (ch)) => Some (Ok (ch)) , Some (Err (_)) => Some (Err (bytes [bytes . len () - 1])) , } }
    };
}

decode_last!()