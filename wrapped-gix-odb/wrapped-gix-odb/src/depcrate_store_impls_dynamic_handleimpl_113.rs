// Generated macro for impl_113 (impl)
macro_rules! Depcrate_store_impls_dynamic_handleimpl_113 {
() => {
// Module: crate::store_impls::dynamic::handle
// Provides: {"impl_113"}
// Dependencies: {}
impl TryFrom < & super :: Store > for super :: Store { type Error = std :: io :: Error ; fn try_from (s : & super :: Store) -> Result < Self , Self :: Error > { super :: Store :: at_opts (s . path () . into () , & mut s . replacements () , crate :: store :: init :: Options { slots : crate :: store :: init :: Slots :: Given (s . files . len () . try_into () . expect ("BUG: too many slots")) , object_hash : Default :: default () , use_multi_pack_index : false , current_dir : s . current_dir . clone () . into () , } ,) } }
};
}
