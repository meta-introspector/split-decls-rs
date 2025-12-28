macro_rules! Lines {
    () => {
        # [doc = " An iterator over line-wise ignore patterns parsed from a buffer."] pub struct Lines < 'a > { lines : bstr :: Lines < 'a > , line_no : usize , # [doc = " Only if `true` we will be able to parse precious files."] support_precious : bool , }
    };
}

Lines!();