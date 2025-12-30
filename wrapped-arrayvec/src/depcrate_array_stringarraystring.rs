// Generated macro for ArrayString (struct)
macro_rules! Depcrate_array_stringArrayString {
() => {
// Module: crate::array_string
// Provides: {"ArrayString"}
// Dependencies: {}
# [doc = " A string with a fixed capacity."] # [doc = ""] # [doc = " The `ArrayString` is a string backed by a fixed size array. It keeps track"] # [doc = " of its length, and is parameterized by `CAP` for the maximum capacity."] # [doc = ""] # [doc = " `CAP` is of type `usize` but is range limited to `u32::MAX` (or `u16` on 16-bit targets);"] # [doc = " attempting to create larger arrayvecs with larger capacity will panic."] # [doc = ""] # [doc = " The string is a contiguous value that you can store directly on the stack"] # [doc = " if needed."] # [derive (Copy)] # [repr (C)] pub struct ArrayString < const CAP : usize > { len : LenUint , xs : [MaybeUninit < u8 > ; CAP] , }
};
}
