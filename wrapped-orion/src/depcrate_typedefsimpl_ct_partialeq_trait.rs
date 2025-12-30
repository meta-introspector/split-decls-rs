// Generated macro for impl_ct_partialeq_trait (macro)
macro_rules! Depcrate_typedefsimpl_ct_partialeq_trait {
() => {
// Module: crate::typedefs
// Provides: {"impl_ct_partialeq_trait"}
// Dependencies: {}
# [doc = " Macro that implements the `PartialEq` trait on a object called `$name` that"] # [doc = " provides a given $bytes_function to return a slice. This `PartialEq` will"] # [doc = " execute in constant-time."] # [doc = ""] # [doc = " This also provides an empty `Eq` implementation."] macro_rules ! impl_ct_partialeq_trait (($ name : ident , $ bytes_function : ident) => (impl PartialEq <$ name > for $ name { fn eq (& self , other : &$ name) -> bool { use subtle :: ConstantTimeEq ; (self .$ bytes_function () . ct_eq (other .$ bytes_function ())) . into () } } impl Eq for $ name { } impl PartialEq <& [u8] > for $ name { fn eq (& self , other : && [u8]) -> bool { use subtle :: ConstantTimeEq ; (self .$ bytes_function () . ct_eq (* other)) . into () } })) ;
};
}
