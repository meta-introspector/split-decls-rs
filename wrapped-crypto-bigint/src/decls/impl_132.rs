macro_rules! deps {
    () => {
        Zero!();
        JacobiSymbol!();
        One!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl Neg for JacobiSymbol { type Output = Self ; fn neg (self) -> Self { match self { Self :: Zero => Self :: Zero , Self :: One => Self :: MinusOne , Self :: MinusOne => Self :: One , } } }
    };
}

impl_132!()