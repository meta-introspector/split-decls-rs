// Generated macro for impl_109 (impl)
macro_rules! Depcrate_abi_exampleimpl_109 {
() => {
// Module: crate::abi_example
// Provides: {"impl_109"}
// Dependencies: {}
impl < T : Serialize + ? Sized > AbiEnumVisitor for T { default fn visit_for_abi (& self , _digester : & mut AbiDigester) -> DigestResult { unreachable ! ("AbiEnumVisitor must be implemented for {}" , type_name ::< T > ()) ; } }
};
}
