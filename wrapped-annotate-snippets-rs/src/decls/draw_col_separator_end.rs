macro_rules! deps {
    () => {
        StyledBuffer!();
        DecorStyle!();
        Renderer!();
        ElementStyle!();
    };
}

macro_rules! draw_col_separator_end {
    () => {
        deps!();
        fn draw_col_separator_end (renderer : & Renderer , buffer : & mut StyledBuffer , line : usize , col : usize) { match renderer . decor_style { DecorStyle :: Ascii => { draw_col_separator_no_space_with_style (buffer , '|' , line , col , ElementStyle :: LineNumber ,) ; } DecorStyle :: Unicode => { draw_col_separator_no_space_with_style (buffer , '╰' , line , col , ElementStyle :: LineNumber ,) ; draw_col_separator_no_space_with_style (buffer , '╴' , line , col + 1 , ElementStyle :: LineNumber ,) ; } } }
    };
}

draw_col_separator_end!();