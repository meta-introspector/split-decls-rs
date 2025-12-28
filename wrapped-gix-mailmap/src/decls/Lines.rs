macro_rules! Lines {
    () => {
        # [doc = " An iterator to parse mailmap lines on-demand."] pub struct Lines < 'a > { lines : bstr :: Lines < 'a > , line_no : usize , }
    };
}

Lines!()