// Generated macro for impl_753 (impl)
macro_rules! Depcrate_types_value_refimpl_753 {
() => {
// Module: crate::types::value_ref
// Provides: {"impl_753"}
// Dependencies: {}
impl < T > From < Option < T > > for ValueRef < '_ > where T : Into < Self > , { # [inline] fn from (s : Option < T >) -> Self { match s { Some (x) => x . into () , None => ValueRef :: Null , } } }
};
}
