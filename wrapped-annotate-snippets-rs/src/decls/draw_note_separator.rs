macro_rules! deps {
    () => {
        ElementStyle!();
        Renderer!();
        StyledBuffer!();
    };
}

macro_rules! draw_note_separator {
    () => {
        deps!();
        fn draw_note_separator (renderer : & Renderer , buffer : & mut StyledBuffer , line : usize , col : usize , is_cont : bool ,) { let chr = renderer . decor_style . note_separator (is_cont) ; buffer . puts (line , col , chr , ElementStyle :: LineNumber) ; }
    };
}

draw_note_separator!()