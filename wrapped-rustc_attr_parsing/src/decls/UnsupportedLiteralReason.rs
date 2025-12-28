macro_rules! UnsupportedLiteralReason {
    () => {
        pub (crate) enum UnsupportedLiteralReason { Generic , CfgString , CfgBoolean , }
    };
}

UnsupportedLiteralReason!();