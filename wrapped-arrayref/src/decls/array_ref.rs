macro_rules! array_ref {
    () => {
        # [doc = " You can use `array_ref` to generate an array reference to a subset"] # [doc = " of a sliceable bit of data (which could be an array, or a slice,"] # [doc = " or a Vec)."] # [doc = ""] # [doc = " **Panics** if the slice is out of bounds."] # [doc = ""] # [doc = " ```"] # [doc = " #[macro_use]"] # [doc = " extern crate arrayref;"] # [doc = ""] # [doc = " fn read_u16(bytes: &[u8; 2]) -> u16 {"] # [doc = "      bytes[0] as u16 + ((bytes[1] as u16) << 8)"] # [doc = " }"] # [doc = " // ..."] # [doc = " # fn main() {"] # [doc = " let data = [0,1,2,3,4,0,6,7,8,9];"] # [doc = " assert_eq!(256, read_u16(array_ref![data,0,2]));"] # [doc = " assert_eq!(4, read_u16(array_ref![data,4,2]));"] # [doc = " # }"] # [doc = " ```"] # [macro_export] macro_rules ! array_ref { ($ arr : expr , $ offset : expr , $ len : expr) => { { { # [inline] const unsafe fn as_array < T > (slice : & [T]) -> & [T ; $ len] { &* (slice . as_ptr () as * const [_ ; $ len]) } let offset = $ offset ; let slice = &$ arr [offset .. offset + $ len] ; # [allow (unused_unsafe)] unsafe { as_array (slice) } } } } ; }
    };
}

array_ref!()