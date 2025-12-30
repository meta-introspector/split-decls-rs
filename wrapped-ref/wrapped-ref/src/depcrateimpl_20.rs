// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl < V : ? Sized > Ref < V > { # [doc = "\n    Get a borrowed wrapper over a borrowed value.\n    "] pub fn new_borrowed < 'a > (value : & 'a V) -> & 'a Ref < V > { unsafe { & * (value as * const _ as * const Ref < V >) } } }
};
}
