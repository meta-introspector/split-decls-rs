// Generated macro for impl_129 (impl)
macro_rules! Depcrate_utilimpl_129 {
() => {
// Module: crate::util
// Provides: {"impl_129"}
// Dependencies: {}
impl < T > UncheckedOptionExt < T > for Option < T > { # [inline] unsafe fn unchecked_unwrap (self) -> T { match self { Some (x) => x , None => unreachable () , } } }
};
}
