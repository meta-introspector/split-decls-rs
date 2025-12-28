macro_rules! deps {
    () => {
        Signature!();
        EcdsaCurve!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl < C : EcdsaCurve > Zeroize for Signature < C > { fn zeroize (& mut self) { self . r = ScalarValue :: ONE ; self . s = ScalarValue :: ONE ; } }
    };
}

impl_160!();