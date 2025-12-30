// Generated macro for impl_110 (impl)
macro_rules! Depcrate_abi_exampleimpl_110 {
() => {
// Module: crate::abi_example
// Provides: {"impl_110"}
// Dependencies: {}
impl < T : Serialize + AbiExample > AbiEnumVisitor for T { default fn visit_for_abi (& self , digester : & mut AbiDigester) -> DigestResult { println ! ("AbiEnumVisitor for T: {}" , type_name ::< T > ()) ; T :: example () . serialize (digester . create_new ()) . map_err (DigestError :: wrap_by_type :: < T >) } }
};
}
