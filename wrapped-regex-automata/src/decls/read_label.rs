macro_rules! deps {
    () => {
        DeserializeError!();
    };
}

macro_rules! read_label {
    () => {
        deps!();
        # [doc = " Reads a NUL terminated label starting at the beginning of the given slice."] # [doc = ""] # [doc = " If a NUL terminated label could not be found, then an error is returned."] # [doc = " Similarly, if a label is found but doesn't match the expected label, then"] # [doc = " an error is returned."] # [doc = ""] # [doc = " Upon success, the total number of bytes read (including padding bytes) is"] # [doc = " returned."] pub (crate) fn read_label (slice : & [u8] , expected_label : & 'static str ,) -> Result < usize , DeserializeError > { let first_nul = slice [.. cmp :: min (slice . len () , 256)] . iter () . position (| & b | b == 0) ; let first_nul = match first_nul { Some (first_nul) => first_nul , None => { return Err (DeserializeError :: generic ("could not find NUL terminated label \
                 at start of serialized object" ,)) ; } } ; let len = first_nul + padding_len (first_nul) ; if slice . len () < len { return Err (DeserializeError :: generic ("could not find properly sized label at start of serialized object")) ; } if expected_label . as_bytes () != & slice [.. first_nul] { return Err (DeserializeError :: label_mismatch (expected_label)) ; } Ok (len) }
    };
}

read_label!()