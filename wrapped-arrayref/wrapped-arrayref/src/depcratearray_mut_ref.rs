// Generated macro for array_mut_ref (macro)
macro_rules! Depcratearray_mut_ref {
() => {
// Module: crate
// Provides: {"array_mut_ref"}
// Dependencies: {}
# [doc = " You can use `array_mut_ref` to generate a mutable array reference"] # [doc = " to a subset of a sliceable bit of data (which could be an array,"] # [doc = " or a slice, or a Vec)."] # [doc = ""] # [doc = " **Panics** if the slice is out of bounds."] # [doc = ""] # [doc = " ```"] # [doc = " #[macro_use]"] # [doc = " extern crate arrayref;"] # [doc = ""] # [doc = " fn write_u16(bytes: &mut [u8; 2], num: u16) {"] # [doc = "      bytes[0] = num as u8;"] # [doc = "      bytes[1] = (num >> 8) as u8;"] # [doc = " }"] # [doc = " // ..."] # [doc = " # fn main() {"] # [doc = " let mut data = [0,1,2,3,4,0,6,7,8,9];"] # [doc = " write_u16(array_mut_ref![data,0,2], 1);"] # [doc = " write_u16(array_mut_ref![data,2,2], 5);"] # [doc = " assert_eq!(*array_ref![data,0,4], [1,0,5,0]);"] # [doc = " *array_mut_ref![data,4,5] = [4,3,2,1,0];"] # [doc = " assert_eq!(data, [1,0,5,0,4,3,2,1,0,9]);"] # [doc = " # }"] # [doc = " ```"] # [macro_export] macro_rules ! array_mut_ref { ($ arr : expr , $ offset : expr , $ len : expr) => { { { # [inline] unsafe fn as_array < T > (slice : & mut [T]) -> & mut [T ; $ len] { & mut * (slice . as_mut_ptr () as * mut [_ ; $ len]) } let offset = $ offset ; let slice = & mut $ arr [offset .. offset + $ len] ; # [allow (unused_unsafe)] unsafe { as_array (slice) } } } } ; }
};
}
