// Generated macro for offset_of (macro)
macro_rules! Depcrate_offset_ofoffset_of {
() => {
// Module: crate::offset_of
// Provides: {"offset_of"}
// Dependencies: {}
# [doc = " Calculates the offset of the specified field from the start of the named struct."] # [doc = ""] # [doc = " ## Examples"] # [doc = " ```"] # [doc = " use memoffset::offset_of;"] # [doc = ""] # [doc = " #[repr(C, packed)]"] # [doc = " struct Foo {"] # [doc = "     a: u32,"] # [doc = "     b: u64,"] # [doc = "     c: [u8; 5]"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(offset_of!(Foo, a), 0);"] # [doc = " assert_eq!(offset_of!(Foo, b), 4);"] # [doc = " ```"] # [doc = ""] # [doc = " ## Notes"] # [doc = " Rust's ABI is unstable, and [type layout can be changed with each"] # [doc = " compilation](https://doc.rust-lang.org/reference/type-layout.html)."] # [doc = ""] # [doc = " Using `offset_of!` with a `repr(Rust)` struct will return the correct offset of the"] # [doc = " specified `field` for a particular compilation, but the exact value may change"] # [doc = " based on the compiler version, concrete struct type, time of day, or rustc's mood."] # [doc = ""] # [doc = " As a result, the value should not be retained and used between different compilations."] # [macro_export (local_inner_macros)] macro_rules ! offset_of { ($ parent : path , $ field : tt) => { _memoffset__offset_of_impl ! ($ parent , $ field) } ; }
};
}
