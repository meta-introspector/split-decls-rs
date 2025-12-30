// Generated macro for impl_152 (impl)
macro_rules! Depcrate_imp_core_atomicimpl_152 {
() => {
// Module: crate::imp::core_atomic
// Provides: {"impl_152"}
// Dependencies: {}
impl < T > core :: ops :: Deref for AtomicPtr < T > { type Target = core :: sync :: atomic :: AtomicPtr < T > ; # [inline] # [cfg_attr (miri , track_caller)] fn deref (& self) -> & Self :: Target { & self . inner } }
};
}
