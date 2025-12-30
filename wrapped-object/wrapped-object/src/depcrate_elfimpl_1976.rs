// Generated macro for impl_1976 (impl)
macro_rules! Depcrate_elfimpl_1976 {
() => {
// Module: crate::elf
// Provides: {"impl_1976"}
// Dependencies: {}
impl < E : Endian > Sym64 < E > { # [doc = " Get the `st_bind` component of the `st_info` field."] # [inline] pub fn st_bind (& self) -> u8 { self . st_info >> 4 } # [doc = " Get the `st_type` component of the `st_info` field."] # [inline] pub fn st_type (& self) -> u8 { self . st_info & 0xf } # [doc = " Set the `st_info` field given the `st_bind` and `st_type` components."] # [inline] pub fn set_st_info (& mut self , st_bind : u8 , st_type : u8) { self . st_info = (st_bind << 4) + (st_type & 0xf) ; } # [doc = " Get the `st_visibility` component of the `st_info` field."] # [inline] pub fn st_visibility (& self) -> u8 { self . st_other & 0x3 } }
};
}
