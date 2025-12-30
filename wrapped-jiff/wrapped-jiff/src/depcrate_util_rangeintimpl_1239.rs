// Generated macro for impl_1239 (impl)
macro_rules! Depcrate_util_rangeintimpl_1239 {
() => {
// Module: crate::util::rangeint
// Provides: {"impl_1239"}
// Dependencies: {}
impl < T , U > TryRInto < U > for T where U : TryRFrom < T > , { # [inline] fn try_rinto (self , what : & 'static str) -> Result < U , Error > { U :: try_rfrom (what , self) } }
};
}
