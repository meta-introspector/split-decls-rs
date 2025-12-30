// Generated macro for private (module)
macro_rules! Depcrate_sealedprivate {
() => {
// Module: crate::sealed
// Provides: {"private"}
// Dependencies: {}
mod private { use digest :: consts :: { U8 , U16 , U24 , U32 } ; pub trait Sealed { } impl Sealed for U8 { } impl Sealed for U16 { } impl Sealed for U24 { } impl Sealed for U32 { } }
};
}
