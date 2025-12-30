// Generated macro for private (module)
macro_rules! Depcrateprivate {
() => {
// Module: crate
// Provides: {"private"}
// Dependencies: {}
mod private { use crate :: { bf16 , f16 } ; use zerocopy :: { FromBytes , Immutable , IntoBytes } ; pub trait SealedHalf : FromBytes + IntoBytes + Immutable { } impl SealedHalf for f16 { } impl SealedHalf for bf16 { } }
};
}
