macro_rules! deps {
    () => {
        ToTokens!();
    };
}

macro_rules! private {
    () => {
        deps!();
        mod private { use crate :: ToTokens ; use proc_macro2 :: extra :: DelimSpan ; use proc_macro2 :: Span ; pub trait Sealed { } impl Sealed for Span { } impl Sealed for DelimSpan { } impl < T : ? Sized + ToTokens > Sealed for T { } }
    };
}

private!();