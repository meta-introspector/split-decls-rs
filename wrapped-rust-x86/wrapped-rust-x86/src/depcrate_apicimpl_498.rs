// Generated macro for impl_498 (impl)
macro_rules! Depcrate_apicimpl_498 {
() => {
// Module: crate::apic
// Provides: {"impl_498"}
// Dependencies: {}
# [allow (clippy :: clippy :: from_over_into)] impl Into < usize > for ApicId { fn into (self) -> usize { match self { ApicId :: XApic (id) => id as usize , ApicId :: X2Apic (id) => id as usize , } } }
};
}
