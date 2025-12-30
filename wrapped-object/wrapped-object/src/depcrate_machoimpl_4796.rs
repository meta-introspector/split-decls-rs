// Generated macro for impl_4796 (impl)
macro_rules! Depcrate_machoimpl_4796 {
() => {
// Module: crate::macho
// Provides: {"impl_4796"}
// Dependencies: {}
impl DyldCacheSlidePointer5 { # [doc = " Whether the pointer is authenticated."] pub fn is_auth (& self) -> bool { ((self . 0 >> 63) & 1) != 0 } # [doc = " The target of the pointer as an offset from the start of the shared cache."] pub fn runtime_offset (& self) -> u64 { self . 0 & 0x3_ffff_ffff } # [doc = " The high 8 bits of the pointer."] # [doc = ""] # [doc = " Only valid if `is_auth` is false."] pub fn high8 (& self) -> u64 { (self . 0 >> 34) & 0xff } # [doc = " The diversity value for authentication."] # [doc = ""] # [doc = " Only valid if `is_auth` is true."] pub fn diversity (& self) -> u16 { ((self . 0 >> 34) & 0xffff) as u16 } # [doc = " Whether to use address diversity for authentication."] # [doc = ""] # [doc = " Only valid if `is_auth` is true."] pub fn addr_div (& self) -> bool { ((self . 0 >> 50) & 1) != 0 } # [doc = " Whether the key is IA or DA."] # [doc = ""] # [doc = " Only valid if `is_auth` is true."] pub fn key_is_data (& self) -> bool { ((self . 0 >> 51) & 1) != 0 } # [doc = " The offset to the next slide pointer in 8-byte units."] # [doc = ""] # [doc = " 0 if no next slide pointer."] pub fn next (& self) -> u64 { (self . 0 >> 52) & 0x7ff } }
};
}
