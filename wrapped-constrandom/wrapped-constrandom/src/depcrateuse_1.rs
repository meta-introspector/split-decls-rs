// Generated macro for use_1 (pub_use)
macro_rules! Depcrateuse_1 {
() => {
// Module: crate
// Provides: {"use_1"}
// Dependencies: {}
# [doc = " # Random constants"] # [doc = " Allows you to insert random constants into your code that will be auto-generated at compile time."] # [doc = " A new value will be generated every time the relevent file is re-built."] # [doc = " # Example"] # [doc = " ```"] # [doc = " use const_random::const_random  ;"] # [doc = " const MY_RANDOM_NUMBER: u32 = const_random!(u32);"] # [doc = " const MY_RANDOM_BYTES: [u8; 32] = const_random!([u8; 32]);"] # [doc = " ```"] # [doc = ""] # [doc = " The following types are supported u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize and [u8; N]."] pub use const_random_macro :: const_random ;
};
}
