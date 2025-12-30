// Generated macro for sealed (module)
macro_rules! Depcrate_rowsealed {
() => {
// Module: crate::row
// Provides: {"sealed"}
// Dependencies: {}
mod sealed { # [doc = " This trait exists just to ensure that the only impls of `trait RowIndex`"] # [doc = " that are allowed are ones in this crate."] pub trait Sealed { } impl Sealed for usize { } impl Sealed for & str { } }
};
}
