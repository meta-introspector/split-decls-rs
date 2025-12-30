// Generated macro for DeriveChoice (struct)
macro_rules! Depcrate_choiceDeriveChoice {
() => {
// Module: crate::choice
// Provides: {"DeriveChoice"}
// Dependencies: {}
# [doc = " Derive the `Choice` trait for an enum."] pub (crate) struct DeriveChoice { # [doc = " Name of the enum type."] ident : Ident , # [doc = " Generics of the enum."] generics : Generics , # [doc = " Variants of this `Choice`."] variants : Vec < ChoiceVariant > , # [doc = " Error type for `DecodeValue` implementation."] error : ErrorType , }
};
}
