macro_rules! deps {
    () => {
        WideEncoding!();
        WideChar!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl WideChar { # [doc = " Returns the length in 8-bit UTF-8 code units."] fn len (& self) -> TextSize { self . end - self . start } # [doc = " Returns the length in UTF-16 or UTF-32 code units."] fn wide_len (& self , enc : WideEncoding) -> u32 { match enc { WideEncoding :: Utf16 => { if self . len () == TextSize :: from (4) { 2 } else { 1 } } WideEncoding :: Utf32 => 1 , } } }
    };
}

impl_23!()