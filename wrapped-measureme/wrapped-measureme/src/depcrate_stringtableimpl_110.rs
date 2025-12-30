// Generated macro for impl_110 (impl)
macro_rules! Depcrate_stringtableimpl_110 {
() => {
// Module: crate::stringtable
// Provides: {"impl_110"}
// Dependencies: {}
impl StringId { pub const INVALID : StringId = StringId (INVALID_STRING_ID) ; # [inline] pub fn new (id : impl Into < u64 >) -> StringId { StringId (id . into ()) } # [inline] pub fn new_virtual (id : impl Into < u64 >) -> StringId { let id = id . into () ; assert ! (id <= MAX_USER_VIRTUAL_STRING_ID) ; StringId (id) } # [inline] pub fn is_virtual (self) -> bool { self . 0 <= METADATA_STRING_ID } # [inline] pub fn as_u64 (self) -> u64 { self . 0 } # [inline] pub fn from_addr (addr : Addr) -> StringId { let id = addr . 0 . checked_add (FIRST_REGULAR_STRING_ID) . unwrap () ; StringId :: new (id) } # [inline] pub fn to_addr (self) -> Addr { Addr (self . 0 . checked_sub (FIRST_REGULAR_STRING_ID) . unwrap ()) } }
};
}
