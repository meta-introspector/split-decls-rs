// Generated macro for impl_133 (impl)
macro_rules! Depcrate_schemaimpl_133 {
() => {
// Module: crate::schema
// Provides: {"impl_133"}
// Dependencies: {}
impl BorshSerialize for BorshSchemaContainer where Declaration : BorshSerialize , BTreeMap < Declaration , Definition > : BorshSerialize , { fn serialize < W : Write > (& self , writer : & mut W) -> IOResult < () > { let declaration = self . declaration () ; let definitions : BTreeMap < & Declaration , & Definition > = self . definitions () . collect () ; BorshSerialize :: serialize (declaration , writer) ? ; BorshSerialize :: serialize (& definitions , writer) ? ; Ok (()) } }
};
}
