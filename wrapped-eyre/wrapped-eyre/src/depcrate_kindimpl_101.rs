// Generated macro for impl_101 (impl)
macro_rules! Depcrate_kindimpl_101 {
() => {
// Module: crate::kind
// Provides: {"impl_101"}
// Dependencies: {}
impl Boxed { # [cfg_attr (track_caller , track_caller)] pub fn new (self , error : Box < dyn StdError + Send + Sync >) -> Report { Report :: from_boxed (error) } }
};
}
