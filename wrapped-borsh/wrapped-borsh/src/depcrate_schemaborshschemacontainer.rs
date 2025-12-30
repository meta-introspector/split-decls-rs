// Generated macro for BorshSchemaContainer (struct)
macro_rules! Depcrate_schemaBorshSchemaContainer {
() => {
// Module: crate::schema
// Provides: {"BorshSchemaContainer"}
// Dependencies: {}
# [doc = " All schema information needed to deserialize a single type."] # [derive (Clone , PartialEq , Eq , Debug)] pub struct BorshSchemaContainer { # [doc = " Declaration of the type."] declaration : Declaration , # [doc = " All definitions needed to deserialize the given type."] definitions : BTreeMap < Declaration , Definition > , }
};
}
