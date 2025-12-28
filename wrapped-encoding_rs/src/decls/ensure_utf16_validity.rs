macro_rules! ensure_utf16_validity {
    () => {
        # [doc = " Replaces unpaired surrogates in the input with the REPLACEMENT CHARACTER."] # [inline] pub fn ensure_utf16_validity (buffer : & mut [u16]) { let mut offset = 0 ; loop { offset += utf16_valid_up_to (& buffer [offset ..]) ; if offset == buffer . len () { return ; } buffer [offset] = 0xFFFD ; offset += 1 ; } }
    };
}

ensure_utf16_validity!()