macro_rules! deps {
    () => {
        Register!();
        ReaderOffset!();
        CfaRule!();
    };
}

macro_rules! impl_237 {
    () => {
        deps!();
        impl < T : ReaderOffset > Default for CfaRule < T > { fn default () -> Self { CfaRule :: RegisterAndOffset { register : Register (0) , offset : 0 , } } }
    };
}

impl_237!()