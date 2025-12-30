// Generated macro for impl_56 (impl)
macro_rules! Depcrate_libtestimpl_56 {
() => {
// Module: crate::libtest
// Provides: {"impl_56"}
// Dependencies: {}
impl TestEvent { # [doc = " Get the name of this test"] pub fn name (& self) -> & str { let (Self :: Started { name } | Self :: Ok { name , .. } | Self :: Ignored { name } | Self :: Failed { name , .. } | Self :: Timeout { name }) = self ; name } # [doc = " Get the stdout of this test, if available."] pub fn stdout (& self) -> Option < & str > { match self { Self :: Ok { stdout , .. } | Self :: Failed { stdout , .. } => stdout . as_deref () , _ => None , } } }
};
}
