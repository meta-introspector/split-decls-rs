macro_rules! deps {
    () => {
        StyledBuffer!();
        ElementStyle!();
    };
}

macro_rules! draw_range {
    () => {
        deps!();
        fn draw_range (buffer : & mut StyledBuffer , symbol : char , line : usize , col_from : usize , col_to : usize , style : ElementStyle ,) { for col in col_from .. col_to { buffer . putc (line , col , symbol , style) ; } }
    };
}

draw_range!();