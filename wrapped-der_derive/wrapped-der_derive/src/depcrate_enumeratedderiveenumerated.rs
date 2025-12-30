// Generated macro for DeriveEnumerated (struct)
macro_rules! Depcrate_enumeratedDeriveEnumerated {
() => {
// Module: crate::enumerated
// Provides: {"DeriveEnumerated"}
// Dependencies: {}
# [doc = " Derive the `Enumerated` trait for an enum."] pub (crate) struct DeriveEnumerated { # [doc = " Name of the enum type."] ident : Ident , # [doc = " Value of the `repr` attribute."] repr : Ident , # [doc = " Whether or not to tag the enum as an integer"] integer : bool , # [doc = " Variants of this enum."] variants : Vec < EnumeratedVariant > , # [doc = " Error type for `DecodeValue` implementation."] error : ErrorType , }
};
}
