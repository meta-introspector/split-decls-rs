// Generated macro for impl_558 (impl)
macro_rules! Depcrate_dbimpl_558 {
() => {
// Module: crate::db
// Provides: {"impl_558"}
// Dependencies: {}
impl :: std :: fmt :: Debug for InternedLifetimeParamId { fn fmt (& self , f : & mut :: std :: fmt :: Formatter < '_ >) -> :: std :: fmt :: Result { f . debug_tuple (stringify ! (InternedLifetimeParamId)) . field (& format_args ! ("{:04x}" , self . 0 . index ())) . finish () } }
};
}
