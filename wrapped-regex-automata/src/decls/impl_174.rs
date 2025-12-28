macro_rules! deps {
    () => {
        StartError!();
        Anchored!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        impl StartError { pub (crate) fn quit (byte : u8) -> StartError { StartError :: Quit { byte } } pub (crate) fn unsupported_anchored (mode : Anchored) -> StartError { StartError :: UnsupportedAnchored { mode } } }
    };
}

impl_174!();