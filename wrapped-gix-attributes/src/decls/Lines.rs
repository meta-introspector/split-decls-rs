macro_rules! Lines {
    () => {
        # [doc = " An iterator over attribute assignments, parsed line by line."] pub struct Lines < 'a > { lines : bstr :: Lines < 'a > , line_no : usize , }
    };
}

Lines!();