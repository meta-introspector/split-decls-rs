// Generated macro for impl_115 (impl)
macro_rules! Depcrate_abi_exampleimpl_115 {
() => {
// Module: crate::abi_example
// Provides: {"impl_115"}
// Dependencies: {}
impl < O : AbiEnumVisitor , E : AbiEnumVisitor > AbiEnumVisitor for Result < O , E > { fn visit_for_abi (& self , digester : & mut AbiDigester) -> DigestResult { println ! ("AbiEnumVisitor for (Result<O, E>): {}" , type_name ::< Self > ()) ; digester . update (& ["enum Result (variants = 2)"]) ; let variant : Self = Result :: Ok (O :: example ()) ; variant . serialize (digester . create_enum_child () ?) ? ; let variant : Self = Result :: Err (E :: example ()) ; variant . serialize (digester . create_enum_child () ?) ? ; digester . create_child () } }
};
}
