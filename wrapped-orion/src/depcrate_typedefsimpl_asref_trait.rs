// Generated macro for impl_asref_trait (macro)
macro_rules! Depcrate_typedefsimpl_asref_trait {
() => {
// Module: crate::typedefs
// Provides: {"impl_asref_trait"}
// Dependencies: {}
# [doc = " Macro that implements the `AsRef<[u8]>` trait on a object called `$name`"] # [doc = " which has fields `value` and `original_length`. This will return the inner"] # [doc = " `value` as a byte slice, and should only be implemented on public types"] # [doc = " which don't have any special protections."] macro_rules ! impl_asref_trait (($ name : ident) => (impl AsRef < [u8] > for $ name { # [inline] fn as_ref (& self) -> & [u8] { self . value [.. self . original_length] . as_ref () } })) ;
};
}
