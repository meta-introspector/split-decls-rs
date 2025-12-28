macro_rules! deps {
    () => {
        ElementStyle!();
        StyledBuffer!();
    };
}

macro_rules! draw_col_separator_no_space_with_style {
    () => {
        deps!();
        fn draw_col_separator_no_space_with_style (buffer : & mut StyledBuffer , chr : char , line : usize , col : usize , style : ElementStyle ,) { buffer . putc (line , col , chr , style) ; }
    };
}

draw_col_separator_no_space_with_style!()