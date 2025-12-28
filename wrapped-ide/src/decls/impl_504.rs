macro_rules! deps {
    () => {
        TextPosition!();
    };
}

macro_rules! impl_504 {
    () => {
        deps!();
        impl TextPosition { pub (crate) fn new (line_index : & LineIndex , offset : TextSize) -> Self { let LineCol { line , col } = line_index . line_col (offset) ; Self { offset , line , col } } }
    };
}

impl_504!();