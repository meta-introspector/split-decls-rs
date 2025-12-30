// Generated macro for impl_7 (impl)
macro_rules! Depcrate_idimpl_7 {
() => {
// Module: crate::id
// Provides: {"impl_7"}
// Dependencies: {}
impl StandardId { # [doc = " CAN ID `0`, the highest priority."] pub const ZERO : Self = Self (0) ; # [doc = " CAN ID `0x7FF`, the lowest priority."] pub const MAX : Self = Self (0x7FF) ; # [doc = " Tries to create a `StandardId` from a raw 16-bit integer."] # [doc = ""] # [doc = " This will return `None` if `raw` is out of range of an 11-bit integer (`> 0x7FF`)."] # [inline] # [must_use] pub const fn new (raw : u16) -> Option < Self > { if raw <= 0x7FF { Some (Self (raw)) } else { None } } # [doc = " Creates a new `StandardId` without checking if it is inside the valid range."] # [doc = ""] # [doc = " # Safety"] # [doc = " Using this method can create an invalid ID and is thus marked as unsafe."] # [inline] # [must_use] pub const unsafe fn new_unchecked (raw : u16) -> Self { Self (raw) } # [doc = " Returns this CAN Identifier as a raw 16-bit integer."] # [inline] # [must_use] pub const fn as_raw (& self) -> u16 { self . 0 } }
};
}
