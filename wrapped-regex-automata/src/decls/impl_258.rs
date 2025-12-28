macro_rules! deps {
    () => {
        Cache!();
        Anchored!();
        StartError!();
        CacheError!();
    };
}

macro_rules! impl_258 {
    () => {
        deps!();
        impl StartError { pub (crate) fn cache (err : CacheError) -> StartError { StartError :: Cache { err } } pub (crate) fn quit (byte : u8) -> StartError { StartError :: Quit { byte } } pub (crate) fn unsupported_anchored (mode : Anchored) -> StartError { StartError :: UnsupportedAnchored { mode } } }
    };
}

impl_258!()