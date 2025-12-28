macro_rules! str_width {
    () => {
        fn str_width (s : & str) -> usize { s . chars () . map (char_width) . sum () }
    };
}

str_width!()