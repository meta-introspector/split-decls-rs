// Generated macro for impl_25 (impl)
macro_rules! Depcrate_macosimpl_25 {
() => {
// Module: crate::macos
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'a > MachHeader < 'a > { unsafe fn from_header_ptr (header : * const libc :: mach_header) -> Option < MachHeader < 'a > > { MachType :: from_header_ptr (header) . and_then (| ty | match ty { MachType :: Mach32 => header . as_ref () . map (MachHeader :: Header32) , MachType :: Mach64 => (header as * const libc :: mach_header_64) . as_ref () . map (MachHeader :: Header64) , }) } }
};
}
