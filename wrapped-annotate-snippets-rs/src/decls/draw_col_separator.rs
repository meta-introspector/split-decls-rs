macro_rules! deps {
    () => {
        Renderer!();
        ElementStyle!();
        StyledBuffer!();
    };
}

macro_rules! draw_col_separator {
    () => {
        deps!();
        fn draw_col_separator (renderer : & Renderer , buffer : & mut StyledBuffer , line : usize , col : usize) { let chr = renderer . decor_style . col_separator () ; buffer . puts (line , col , & format ! ("{chr} ") , ElementStyle :: LineNumber) ; }
    };
}

draw_col_separator!()