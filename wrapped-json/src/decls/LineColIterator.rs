macro_rules! deps {
    () => {
        Index!();
    };
}

macro_rules! LineColIterator {
    () => {
        deps!();
        pub struct LineColIterator < I > { iter : I , # [doc = " Index of the current line. Characters in the first line of the input"] # [doc = " (before the first newline character) are in line 1."] line : usize , # [doc = " Index of the current column. The first character in the input and any"] # [doc = " characters immediately following a newline character are in column 1."] # [doc = " The column is 0 immediately after a newline character has been read."] col : usize , # [doc = " Byte offset of the start of the current line. This is the sum of lengths"] # [doc = " of all previous lines. Keeping track of things this way allows efficient"] # [doc = " computation of the current line, column, and byte offset while only"] # [doc = " updating one of the counters in `next()` in the common case."] start_of_line : usize , }
    };
}

LineColIterator!()