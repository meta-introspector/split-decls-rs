// Generated macro for impl_112 (impl)
macro_rules! Depcrate_abi_exampleimpl_112 {
() => {
// Module: crate::abi_example
// Provides: {"impl_112"}
// Dependencies: {}
impl < T : Serialize + TransparentAsHelper > AbiEnumVisitor for & T { default fn visit_for_abi (& self , digester : & mut AbiDigester) -> DigestResult { println ! ("AbiEnumVisitor for (TransparentAsHelper): {}" , type_name ::< T > ()) ; self . serialize (digester . create_new ()) . map_err (DigestError :: wrap_by_type :: < T >) } }
};
}
