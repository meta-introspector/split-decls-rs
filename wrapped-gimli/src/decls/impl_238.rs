macro_rules! deps {
    () => {
        CfaRule!();
        ReaderOffset!();
        Register!();
    };
}

macro_rules! impl_238 {
    () => {
        deps!();
        impl < T : ReaderOffset > CfaRule < T > { fn is_default (& self) -> bool { match * self { CfaRule :: RegisterAndOffset { register , offset } => { register == Register (0) && offset == 0 } _ => false , } } }
    };
}

impl_238!()