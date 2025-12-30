// Generated macro for impl_327 (impl)
macro_rules! Depcrate_upgradeimpl_327 {
() => {
// Module: crate::upgrade
// Provides: {"impl_327"}
// Dependencies: {}
impl OnUpgrade { pub (super) fn none () -> Self { OnUpgrade { rx : None } } # [cfg (all (any (feature = "client" , feature = "server") , feature = "http1"))] pub (super) fn is_none (& self) -> bool { self . rx . is_none () } }
};
}
