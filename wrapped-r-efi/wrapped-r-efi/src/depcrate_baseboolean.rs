// Generated macro for Boolean (struct)
macro_rules! Depcrate_baseBoolean {
() => {
// Module: crate::base
// Provides: {"Boolean"}
// Dependencies: {}
# [doc = " Boolean Type"] # [doc = ""] # [doc = " This boolean type works very similar to the rust primitive type of [`bool`]. However, the rust"] # [doc = " primitive type has no stable ABI, hence we provide this type to represent booleans on the FFI"] # [doc = " interface."] # [doc = ""] # [doc = " UEFI defines booleans to be 1-byte integers, which can only have the values of `0` or `1`."] # [doc = " However, in practice anything non-zero is considered `true` by nearly all UEFI systems. Hence,"] # [doc = " this type implements a boolean over `u8` and maps `0` to `false`, everything else to `true`."] # [doc = ""] # [doc = " The binary representation of this type is ABI. That is, you are allowed to transmute from and"] # [doc = " to `u8`. Furthermore, this type never modifies its binary representation. If it was"] # [doc = " initialized as, or transmuted from, a specific integer value, this value will be retained."] # [doc = " However, on the rust side you will never see the integer value. It instead behaves truly as a"] # [doc = " boolean. If you need access to the integer value, you have to transmute it back to `u8`."] # [repr (C)] # [derive (Clone , Copy , Debug)] pub struct Boolean (u8) ;
};
}
