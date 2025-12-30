// Generated macro for impl_132 (impl)
macro_rules! Depcrate_schemaimpl_132 {
() => {
// Module: crate::schema
// Provides: {"impl_132"}
// Dependencies: {}
impl BorshSchemaContainer { pub fn new (declaration : Declaration , definitions : BTreeMap < Declaration , Definition >) -> Self { Self { declaration , definitions , } } # [doc = " generate [BorshSchemaContainer] for type `T`"] pub fn for_type < T : BorshSchema + ? Sized > () -> Self { let mut definitions = Default :: default () ; T :: add_definitions_recursively (& mut definitions) ; Self :: new (T :: declaration () , definitions) } pub fn declaration (& self) -> & Declaration { & self . declaration } pub fn definitions (& self) -> impl Iterator < Item = (& '_ Declaration , & '_ Definition) > { self . definitions . iter () } pub fn get_definition < Q > (& self , declaration : & Q) -> Option < & Definition > where Declaration : Borrow < Q > , Q : Ord + ? Sized , { self . definitions . get (declaration) } pub fn get_mut_definition < Q > (& mut self , declaration : & Q) -> Option < & mut Definition > where Declaration : Borrow < Q > , Q : Ord + ? Sized , { self . definitions . get_mut (declaration) } pub fn insert_definition (& mut self , declaration : Declaration , definition : Definition ,) -> Option < Definition > { self . definitions . insert (declaration , definition) } pub fn remove_definition < Q > (& mut self , declaration : & Q) -> Option < Definition > where Declaration : Borrow < Q > , Q : Ord + ? Sized , { self . definitions . remove (declaration) } }
};
}
