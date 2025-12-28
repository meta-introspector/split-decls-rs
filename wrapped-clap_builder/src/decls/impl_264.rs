macro_rules! deps {
    () => {
        ValueParserInner!();
        ValueParser!();
    };
}

macro_rules! impl_264 {
    () => {
        deps!();
        impl Clone for ValueParser { fn clone (& self) -> Self { Self (match & self . 0 { ValueParserInner :: Bool => ValueParserInner :: Bool , ValueParserInner :: String => ValueParserInner :: String , ValueParserInner :: OsString => ValueParserInner :: OsString , ValueParserInner :: PathBuf => ValueParserInner :: PathBuf , ValueParserInner :: Other (o) => ValueParserInner :: Other (o . clone_any ()) , }) } }
    };
}

impl_264!();