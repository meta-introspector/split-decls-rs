macro_rules! OverflowingBinHexSign {
    () => {
        pub (crate) enum OverflowingBinHexSign { Positive , Negative , }
    };
}

OverflowingBinHexSign!();