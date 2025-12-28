macro_rules! deps {
    () => {
        CharAccumulator!();
        AsciiParser!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl CharAccumulator for AsciiParser { fn add (& mut self , _byte : u8) -> Option < char > { unreachable ! ("multi-byte UTF8 characters are unsupported") } }
    };
}

impl_12!()