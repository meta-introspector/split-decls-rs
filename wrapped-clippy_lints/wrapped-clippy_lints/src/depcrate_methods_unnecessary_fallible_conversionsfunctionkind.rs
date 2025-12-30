// Generated macro for FunctionKind (enum)
macro_rules! Depcrate_methods_unnecessary_fallible_conversionsFunctionKind {
() => {
// Module: crate::methods::unnecessary_fallible_conversions
// Provides: {"FunctionKind"}
// Dependencies: {}
# [doc = " What function is being called and whether that call is written as a method call or a function"] # [doc = " call"] # [derive (Copy , Clone)] # [expect (clippy :: enum_variant_names)] enum FunctionKind { # [doc = " `T::try_from(U)`"] TryFromFunction (Option < SpansKind >) , # [doc = " `t.try_into()`"] TryIntoMethod , # [doc = " `U::try_into(t)`"] TryIntoFunction (Option < SpansKind >) , }
};
}
