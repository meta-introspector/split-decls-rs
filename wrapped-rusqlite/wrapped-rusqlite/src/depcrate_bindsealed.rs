// Generated macro for sealed (module)
macro_rules! Depcrate_bindsealed {
() => {
// Module: crate::bind
// Provides: {"sealed"}
// Dependencies: {}
mod sealed { use std :: ffi :: CStr ; # [doc = " This trait exists just to ensure that the only impls of `trait BindIndex`"] # [doc = " that are allowed are ones in this crate."] pub trait Sealed { } impl Sealed for usize { } impl Sealed for & str { } impl Sealed for & CStr { } }
};
}
