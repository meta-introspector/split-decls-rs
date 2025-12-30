// Generated macro for impl_64 (impl)
macro_rules! Depcrate_utilimpl_64 {
() => {
// Module: crate::util
// Provides: {"impl_64"}
// Dependencies: {}
impl < T > UncheckedOptionExt < T > for Option < T > { # [inline] unsafe fn unchecked_unwrap (self) -> T { match self { Some (x) => x , None => unreachable () , } } }
};
}
