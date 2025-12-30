// Generated macro for impl_137 (impl)
macro_rules! Depcrate_schemaimpl_137 {
() => {
// Module: crate::schema
// Provides: {"impl_137"}
// Dependencies: {}
impl BorshSchema for BorshSchemaContainer where Declaration : BorshSchema , BTreeMap < Declaration , Definition > : BorshSchema , { fn declaration () -> Declaration { "BorshSchemaContainer" . to_string () } fn add_definitions_recursively (definitions : & mut BTreeMap < Declaration , Definition >) { let fields = Fields :: NamedFields (< [_] > :: into_vec (Box :: new ([("declaration" . to_string () , < Declaration as BorshSchema > :: declaration () ,) , ("definitions" . to_string () , < BTreeMap < Declaration , Definition > as BorshSchema > :: declaration () ,) ,]))) ; let definition = Definition :: Struct { fields } ; add_definition (< Self as BorshSchema > :: declaration () , definition , definitions ,) ; < Declaration as BorshSchema > :: add_definitions_recursively (definitions) ; < BTreeMap < Declaration , Definition > as BorshSchema > :: add_definitions_recursively (definitions ,) ; } }
};
}
