// Generated macro for impl_134 (impl)
macro_rules! Depcrate_schemaimpl_134 {
() => {
// Module: crate::schema
// Provides: {"impl_134"}
// Dependencies: {}
impl BorshDeserialize for BorshSchemaContainer where Declaration : BorshDeserialize , BTreeMap < Declaration , Definition > : BorshDeserialize , { fn deserialize_reader < R : Read > (reader : & mut R) -> IOResult < Self > { let declaration : Declaration = BorshDeserialize :: deserialize_reader (reader) ? ; let definitions : BTreeMap < Declaration , Definition > = BorshDeserialize :: deserialize_reader (reader) ? ; Ok (Self :: new (declaration , definitions)) } }
};
}
