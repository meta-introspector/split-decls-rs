macro_rules! deps {
    () => {
        DecorStyle!();
        Renderer!();
        StyledBuffer!();
        ElementStyle!();
    };
}

macro_rules! draw_line_separator {
    () => {
        deps!();
        fn draw_line_separator (renderer : & Renderer , buffer : & mut StyledBuffer , line : usize , col : usize) { let (column , dots) = match renderer . decor_style { DecorStyle :: Ascii => (0 , "...") , DecorStyle :: Unicode => (col - 2 , "‡") , } ; buffer . puts (line , column , dots , ElementStyle :: LineNumber) ; }
    };
}

draw_line_separator!()