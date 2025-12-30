// Generated macro for impl_75 (impl)
macro_rules! Depcrate_kv_sourceimpl_75 {
() => {
// Module: crate::kv::source
// Provides: {"impl_75"}
// Dependencies: {}
impl < const N : usize , S > Source for [S ; N] where S : Source , { fn visit < 'kvs > (& 'kvs self , visitor : & mut dyn VisitSource < 'kvs >) -> Result < () , Error > { Source :: visit (self as & [_] , visitor) } fn get (& self , key : Key) -> Option < Value < '_ > > { Source :: get (self as & [_] , key) } fn count (& self) -> usize { Source :: count (self as & [_]) } }
};
}
