// Generated macro for use_25 (pub_use)
macro_rules! Depcrateuse_25 {
() => {
// Module: crate
// Provides: {"use_25"}
// Dependencies: {}
# [doc = " Generates a bitflags structure that can be formatted with defmt."] # [doc = ""] # [doc = " This macro is a wrapper around the [`bitflags!`] crate, and provides an (almost) identical"] # [doc = " interface. Refer to [its documentation] for an explanation of the syntax."] # [doc = ""] # [doc = " [its documentation]: https://docs.rs/bitflags/1/bitflags/"] # [doc = ""] # [doc = " # Limitations"] # [doc = ""] # [doc = " This macro only supports bitflags structs represented as one of Rust's built-in unsigned integer"] # [doc = " types (`u8`, `u16`, `u32`, `u64`, or `u128`). Custom types are not supported. This restriction"] # [doc = " is necessary to support defmt's efficient encoding."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " The example from the bitflags crate works as-is:"] # [doc = ""] # [doc = " ```"] # [doc = " defmt::bitflags! {"] # [doc = "     struct Flags: u32 {"] # [doc = "         const A = 0b00000001;"] # [doc = "         const B = 0b00000010;"] # [doc = "         const C = 0b00000100;"] # [doc = "         const ABC = Self::A.bits | Self::B.bits | Self::C.bits;"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " defmt::info!(\"Flags::ABC: {}\", Flags::ABC);"] # [doc = " defmt::info!(\"Flags::empty(): {}\", Flags::empty());"] # [doc = " ```"] pub use defmt10 :: bitflags ;
};
}
