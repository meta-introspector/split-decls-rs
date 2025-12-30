// Generated macro for impl_179 (impl)
macro_rules! Depcrate_writeimpl_179 {
() => {
// Module: crate::write
// Provides: {"impl_179"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl Write for Vec < u8 > { type Error = error :: Error ; fn write_all (& mut self , buf : & [u8]) -> Result < () , Self :: Error > { self . extend_from_slice (buf) ; Ok (()) } }
};
}
