// Generated macro for use_14 (pub_use)
macro_rules! Depcrateuse_14 {
() => {
// Module: crate
// Provides: {"use_14"}
// Dependencies: {}
# [doc = " Creates an interned string ([`Str`]) from a string literal."] # [doc = ""] # [doc = " This must be called on a string literal, and will allocate the literal in the object file. At"] # [doc = " runtime, only a small string index is required to refer to the string, represented as the"] # [doc = " [`Str`] type."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " let interned = defmt::intern!(\"long string literal taking up little space\");"] # [doc = " ```"] # [doc = ""] # [doc = " [`Str`]: struct.Str.html"] pub use defmt10 :: intern ;
};
}
