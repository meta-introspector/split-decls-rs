// Generated macro for impl_23 (impl)
macro_rules! Depcrate_macosimpl_23 {
() => {
// Module: crate::macos
// Provides: {"impl_23"}
// Dependencies: {}
impl MachType { unsafe fn from_header_ptr (header : * const libc :: mach_header) -> Option < MachType > { header . as_ref () . and_then (| header | match header . magic { libc :: MH_MAGIC => Some (MachType :: Mach32) , libc :: MH_MAGIC_64 => Some (MachType :: Mach64) , _ => None , }) } }
};
}
