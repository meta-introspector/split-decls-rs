// Generated macro for impl_111 (impl)
macro_rules! Depcrate_abi_exampleimpl_111 {
() => {
// Module: crate::abi_example
// Provides: {"impl_111"}
// Dependencies: {}
impl < T : Serialize + ? Sized + AbiEnumVisitor > AbiEnumVisitor for & T { default fn visit_for_abi (& self , digester : & mut AbiDigester) -> DigestResult { println ! ("AbiEnumVisitor for &T: {}" , type_name ::< T > ()) ; T :: visit_for_abi (self , digester) } }
};
}
