// Generated macro for impl_740 (impl)
macro_rules! Depcrate_types_valueimpl_740 {
() => {
// Module: crate::types::value
// Provides: {"impl_740"}
// Dependencies: {}
impl < T > From < Option < T > > for Value where T : Into < Self > , { # [inline] fn from (v : Option < T >) -> Self { match v { Some (x) => x . into () , None => Self :: Null , } } }
};
}
