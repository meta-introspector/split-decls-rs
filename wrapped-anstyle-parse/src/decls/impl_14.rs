macro_rules! deps {
    () => {
        Utf8Parser!();
        VtUtf8Receiver!();
        CharAccumulator!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        # [cfg (feature = "utf8")] impl CharAccumulator for Utf8Parser { fn add (& mut self , byte : u8) -> Option < char > { let mut c = None ; let mut receiver = VtUtf8Receiver (& mut c) ; self . utf8_parser . advance (& mut receiver , byte) ; c } }
    };
}

impl_14!()