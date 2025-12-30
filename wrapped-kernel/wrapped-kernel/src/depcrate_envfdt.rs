// Generated macro for fdt (function)
macro_rules! Depcrate_envfdt {
() => {
// Module: crate::env
// Provides: {"fdt"}
// Dependencies: {}
pub fn fdt () -> Option < Fdt < 'static > > { boot_info () . hardware_info . device_tree . map (| fdt | { let ptr = ptr :: with_exposed_provenance (fdt . get () . try_into () . unwrap ()) ; unsafe { Fdt :: from_ptr (ptr) . unwrap () } }) }
};
}
