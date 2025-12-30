// Generated macro for impl_307 (impl)
macro_rules! Depcrate_valueimpl_307 {
() => {
// Module: crate::value
// Provides: {"impl_307"}
// Dependencies: {}
impl < T > From < Option < T > > for ValueKind where T : Into < Self > , { fn from (value : Option < T >) -> Self { match value { Some (value) => value . into () , None => Self :: Nil , } } }
};
}
