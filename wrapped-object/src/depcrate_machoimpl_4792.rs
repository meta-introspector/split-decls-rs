// Generated macro for impl_4792 (impl)
macro_rules! Depcrate_machoimpl_4792 {
() => {
// Module: crate::macho
// Provides: {"impl_4792"}
// Dependencies: {}
impl DyldCacheSlidePointer3 { # [doc = " Whether the pointer is authenticated."] pub fn is_auth (& self) -> bool { ((self . 0 >> 63) & 1) != 0 } # [doc = " The target of the pointer."] # [doc = ""] # [doc = " Only valid if `is_auth` is false."] pub fn target (& self) -> u64 { self . 0 & ((1 << 43) - 1) } # [doc = " The high 8 bits of the pointer."] # [doc = ""] # [doc = " Only valid if `is_auth` is false."] pub fn high8 (& self) -> u64 { (self . 0 >> 43) & 0xff } # [doc = " The target of the pointer as an offset from the start of the shared cache."] # [doc = ""] # [doc = " Only valid if `is_auth` is true."] pub fn runtime_offset (& self) -> u64 { self . 0 & ((1 << 32) - 1) } # [doc = " The diversity value for authentication."] # [doc = ""] # [doc = " Only valid if `is_auth` is true."] pub fn diversity (& self) -> u16 { ((self . 0 >> 32) & 0xffff) as u16 } # [doc = " Whether to use address diversity for authentication."] # [doc = ""] # [doc = " Only valid if `is_auth` is true."] pub fn addr_div (& self) -> bool { ((self . 0 >> 48) & 1) != 0 } # [doc = " The key for authentication."] # [doc = ""] # [doc = " Only valid if `is_auth` is true."] pub fn key (& self) -> u8 { ((self . 0 >> 49) & 3) as u8 } # [doc = " The offset to the next slide pointer in 8-byte units."] # [doc = ""] # [doc = " 0 if no next slide pointer."] pub fn next (& self) -> u64 { (self . 0 >> 51) & ((1 << 11) - 1) } }
};
}
