macro_rules! deps {
    () => {
        StyledBuffer!();
        Renderer!();
        DecorStyle!();
        ElementStyle!();
    };
}

macro_rules! draw_col_separator_start {
    () => {
        deps!();
        fn draw_col_separator_start (renderer : & Renderer , buffer : & mut StyledBuffer , line : usize , col : usize ,) { match renderer . decor_style { DecorStyle :: Ascii => { draw_col_separator_no_space_with_style (buffer , '|' , line , col , ElementStyle :: LineNumber ,) ; } DecorStyle :: Unicode => { draw_col_separator_no_space_with_style (buffer , '╭' , line , col , ElementStyle :: LineNumber ,) ; draw_col_separator_no_space_with_style (buffer , '╴' , line , col + 1 , ElementStyle :: LineNumber ,) ; } } }
    };
}

draw_col_separator_start!();