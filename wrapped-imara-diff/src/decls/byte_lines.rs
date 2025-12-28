macro_rules! deps {
    () => {
        ByteLines!();
        TokenSource!();
    };
}

macro_rules! byte_lines {
    () => {
        deps!();
        # [doc = " Returns a [`TokenSource`] that uses the lines in `data` as Tokens. The newline"] # [doc = " separator (`\\r\\n` or `\\n`) is included in the emitted tokens. This means that changing"] # [doc = " the newline separator from `\\r\\n` to `\\n` (or omitting it fully on the last line) is"] # [doc = " detected when computing a [`Diff`](crate::Diff)."] pub fn byte_lines (data : & [u8]) -> ByteLines < '_ > { ByteLines (data) }
    };
}

byte_lines!()