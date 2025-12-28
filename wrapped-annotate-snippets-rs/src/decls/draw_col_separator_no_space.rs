macro_rules! deps {
    () => {
        StyledBuffer!();
        Renderer!();
        ElementStyle!();
    };
}

macro_rules! draw_col_separator_no_space {
    () => {
        deps!();
        fn draw_col_separator_no_space (renderer : & Renderer , buffer : & mut StyledBuffer , line : usize , col : usize ,) { let chr = renderer . decor_style . col_separator () ; draw_col_separator_no_space_with_style (buffer , chr , line , col , ElementStyle :: LineNumber) ; }
    };
}

draw_col_separator_no_space!()