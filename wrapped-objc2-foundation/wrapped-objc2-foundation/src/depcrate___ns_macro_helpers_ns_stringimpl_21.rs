// Generated macro for impl_21 (impl)
macro_rules! Depcrate___ns_macro_helpers_ns_stringimpl_21 {
() => {
// Module: crate::__ns_macro_helpers::ns_string
// Provides: {"impl_21"}
// Dependencies: {}
impl CFConstString { const FLAGS_ASCII : u32 = 0x07_C8 ; const FLAGS_UTF16 : u32 = 0x07_D0 ; pub const unsafe fn new_ascii (isa : & 'static AnyClass , data : & 'static [u8]) -> Self { Self { isa , cfinfo : Self :: FLAGS_ASCII , # [cfg (target_pointer_width = "64")] _rc : 0 , data : data . as_ptr () . cast () , len : data . len () - 1 , } } pub const unsafe fn new_utf16 (isa : & 'static AnyClass , data : & 'static [u16]) -> Self { Self { isa , cfinfo : Self :: FLAGS_UTF16 , # [cfg (target_pointer_width = "64")] _rc : 0 , data : data . as_ptr () . cast () , len : data . len () - 1 , } } # [inline] pub const fn as_nsstring_const (& self) -> & NSString { let ptr : * const Self = self ; unsafe { & * ptr . cast :: < NSString > () } } # [inline] pub fn as_nsstring (& self) -> & NSString { self . as_nsstring_const () } }
};
}
