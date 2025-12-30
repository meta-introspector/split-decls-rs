// Generated macro for impl_113 (impl)
macro_rules! Depcrate_abi_exampleimpl_113 {
() => {
// Module: crate::abi_example
// Provides: {"impl_113"}
// Dependencies: {}
impl < T : Serialize + TransparentAsHelper + EvenAsOpaque > AbiEnumVisitor for & T { default fn visit_for_abi (& self , digester : & mut AbiDigester) -> DigestResult { let type_name = type_name :: < T > () ; let matcher = T :: TYPE_NAME_MATCHER ; println ! ("AbiEnumVisitor for (EvenAsOpaque): {type_name}: matcher: {matcher}") ; self . serialize (digester . create_new_opaque (matcher)) . map_err (DigestError :: wrap_by_type :: < T >) } }
};
}
