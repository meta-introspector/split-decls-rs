macro_rules! deps {
    () => {
        StyledBuffer!();
        Renderer!();
        DecorStyle!();
        ElementStyle!();
    };
}

macro_rules! draw_multiline_line {
    () => {
        deps!();
        fn draw_multiline_line (renderer : & Renderer , buffer : & mut StyledBuffer , line : usize , offset : usize , depth : usize , style : ElementStyle ,) { let chr = match (style , renderer . decor_style) { (ElementStyle :: UnderlinePrimary | ElementStyle :: LabelPrimary , DecorStyle :: Ascii) => '|' , (_ , DecorStyle :: Ascii) => '|' , (ElementStyle :: UnderlinePrimary | ElementStyle :: LabelPrimary , DecorStyle :: Unicode) => '┃' , (_ , DecorStyle :: Unicode) => '│' , } ; buffer . putc (line , offset + depth - 1 , chr , style) ; }
    };
}

draw_multiline_line!();