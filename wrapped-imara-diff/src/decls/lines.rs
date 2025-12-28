macro_rules! deps {
    () => {
        Lines!();
        TokenSource!();
        ByteLines!();
    };
}

macro_rules! lines {
    () => {
        deps!();
        # [doc = " Returns a [`TokenSource`] that uses the lines in `data` as Tokens. The newline"] # [doc = " separator (`\\r\\n` or `\\n`) is included in the emitted tokens. This means that changing"] # [doc = " the newline separator from `\\r\\n` to `\\n` (or omitting it fully on the last line) is"] # [doc = " detected by [`Diff`](crate::Diff)."] pub fn lines (data : & str) -> Lines < '_ > { Lines (ByteLines (data . as_bytes ())) }
    };
}

lines!();