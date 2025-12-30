// Generated macro for impl_for_range (macro)
macro_rules! Depcrate_schemaimpl_for_range {
() => {
// Module: crate::schema
// Provides: {"impl_for_range"}
// Dependencies: {}
macro_rules ! impl_for_range { ($ type : ident , $ ($ name : ident) ,*) => { impl < T : BorshSchema > BorshSchema for core :: ops ::$ type < T > { fn add_definitions_recursively (definitions : & mut BTreeMap < Declaration , Definition >) { let decl = T :: declaration () ; let fields = Fields :: NamedFields (vec ! [$ ((FieldName :: from (stringify ! ($ name)) , decl . clone ())) ,*]) ; let def = Definition :: Struct { fields } ; add_definition (Self :: declaration () , def , definitions) ; T :: add_definitions_recursively (definitions) ; } fn declaration () -> Declaration { format ! ("{}<{}>" , stringify ! ($ type) , T :: declaration ()) } } } ; }
};
}
