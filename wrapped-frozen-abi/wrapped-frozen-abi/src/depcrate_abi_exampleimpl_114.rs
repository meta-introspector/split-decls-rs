// Generated macro for impl_114 (impl)
macro_rules! Depcrate_abi_exampleimpl_114 {
() => {
// Module: crate::abi_example
// Provides: {"impl_114"}
// Dependencies: {}
impl < T : AbiEnumVisitor > AbiEnumVisitor for Option < T > { fn visit_for_abi (& self , digester : & mut AbiDigester) -> DigestResult { println ! ("AbiEnumVisitor for (Option<T>): {}" , type_name ::< Self > ()) ; let variant : Self = Option :: Some (T :: example ()) ; variant . serialize (digester . create_new ()) } }
};
}
