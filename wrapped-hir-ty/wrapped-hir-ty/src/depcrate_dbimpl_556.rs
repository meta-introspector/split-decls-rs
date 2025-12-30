// Generated macro for impl_556 (impl)
macro_rules! Depcrate_dbimpl_556 {
() => {
// Module: crate::db
// Provides: {"impl_556"}
// Dependencies: {}
impl :: std :: fmt :: Debug for InternedTypeOrConstParamId { fn fmt (& self , f : & mut :: std :: fmt :: Formatter < '_ >) -> :: std :: fmt :: Result { f . debug_tuple (stringify ! (InternedTypeOrConstParamId)) . field (& format_args ! ("{:04x}" , self . 0 . index ())) . finish () } }
};
}
