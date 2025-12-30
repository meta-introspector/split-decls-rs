// Generated macro for private (module)
macro_rules! Depcrate_spannedprivate {
() => {
// Module: crate::spanned
// Provides: {"private"}
// Dependencies: {}
mod private { use crate :: ToTokens ; use proc_macro2 :: extra :: DelimSpan ; use proc_macro2 :: Span ; pub trait Sealed { } impl Sealed for Span { } impl Sealed for DelimSpan { } impl < T : ? Sized + ToTokens > Sealed for T { } }
};
}
