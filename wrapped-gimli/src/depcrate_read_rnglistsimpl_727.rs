// Generated macro for impl_727 (impl)
macro_rules! Depcrate_read_rnglistsimpl_727 {
() => {
// Module: crate::read::rnglists
// Provides: {"impl_727"}
// Dependencies: {}
impl Range { # [doc = " Add a base address to this range."] # [inline] pub (crate) fn add_base_address (& mut self , base_address : u64 , address_size : u8) { self . begin = base_address . wrapping_add_sized (self . begin , address_size) ; self . end = base_address . wrapping_add_sized (self . end , address_size) ; } }
};
}
