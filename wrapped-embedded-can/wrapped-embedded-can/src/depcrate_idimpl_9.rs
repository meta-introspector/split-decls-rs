// Generated macro for impl_9 (impl)
macro_rules! Depcrate_idimpl_9 {
() => {
// Module: crate::id
// Provides: {"impl_9"}
// Dependencies: {}
impl ExtendedId { # [doc = " CAN ID `0`, the highest priority."] pub const ZERO : Self = Self (0) ; # [doc = " CAN ID `0x1FFFFFFF`, the lowest priority."] pub const MAX : Self = Self (0x1FFF_FFFF) ; # [doc = " Tries to create a `ExtendedId` from a raw 32-bit integer."] # [doc = ""] # [doc = " This will return `None` if `raw` is out of range of an 29-bit integer (`> 0x1FFF_FFFF`)."] # [inline] # [must_use] pub const fn new (raw : u32) -> Option < Self > { if raw <= 0x1FFF_FFFF { Some (Self (raw)) } else { None } } # [doc = " Creates a new `ExtendedId` without checking if it is inside the valid range."] # [doc = ""] # [doc = " # Safety"] # [doc = " Using this method can create an invalid ID and is thus marked as unsafe."] # [inline] # [must_use] pub const unsafe fn new_unchecked (raw : u32) -> Self { Self (raw) } # [doc = " Returns this CAN Identifier as a raw 32-bit integer."] # [inline] # [must_use] pub const fn as_raw (& self) -> u32 { self . 0 } # [doc = " Returns the Base ID part of this extended identifier."] # [must_use] pub fn standard_id (& self) -> StandardId { StandardId ((self . 0 >> 18) as u16) } }
};
}
