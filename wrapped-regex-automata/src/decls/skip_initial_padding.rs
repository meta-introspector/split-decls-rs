macro_rules! skip_initial_padding {
    () => {
        # [doc = " Reads a possibly empty amount of padding, up to 7 bytes, from the beginning"] # [doc = " of the given slice. All padding bytes must be NUL bytes."] # [doc = ""] # [doc = " This is useful because it can be theoretically necessary to pad the"] # [doc = " beginning of a serialized object with NUL bytes to ensure that it starts"] # [doc = " at a correctly aligned address. These padding bytes should come immediately"] # [doc = " before the label."] # [doc = ""] # [doc = " This returns the number of bytes read from the given slice."] pub (crate) fn skip_initial_padding (slice : & [u8]) -> usize { let mut nread = 0 ; while nread < 7 && nread < slice . len () && slice [nread] == 0 { nread += 1 ; } nread }
    };
}

skip_initial_padding!();