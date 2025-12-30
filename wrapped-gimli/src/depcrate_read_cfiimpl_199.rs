// Generated macro for impl_199 (impl)
macro_rules! Depcrate_read_cfiimpl_199 {
() => {
// Module: crate::read::cfi
// Provides: {"impl_199"}
// Dependencies: {}
impl < R : Reader > DebugFrame < R > { # [doc = " Set the size of a target address in bytes."] # [doc = ""] # [doc = " This defaults to the native word size."] # [doc = " This is only used if the CIE version is less than 4."] pub fn set_address_size (& mut self , address_size : u8) { self . address_size = address_size } # [doc = " Set the vendor extensions to use."] # [doc = ""] # [doc = " This defaults to `Vendor::Default`."] pub fn set_vendor (& mut self , vendor : Vendor) { self . vendor = vendor ; } }
};
}
