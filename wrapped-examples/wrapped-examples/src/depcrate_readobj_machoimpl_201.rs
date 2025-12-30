// Generated macro for impl_201 (impl)
macro_rules! Depcrate_readobj_machoimpl_201 {
() => {
// Module: crate::readobj::macho
// Provides: {"impl_201"}
// Dependencies: {}
impl < 'a > PrinterMachoExt for Printer < 'a > { fn field_version (& mut self , name : & str , value : u32) { let major = (value >> 16) & 0xFFFF ; let minor = (value >> 8) & 0xFF ; let update = value & 0xFF ; self . field (name , format ! ("{}.{}.{}" , major , minor , update)) ; } }
};
}
