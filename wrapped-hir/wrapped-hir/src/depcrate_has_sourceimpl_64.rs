// Generated macro for impl_64 (impl)
macro_rules! Depcrate_has_sourceimpl_64 {
() => {
// Module: crate::has_source
// Provides: {"impl_64"}
// Dependencies: {}
impl HasSource for Adt { type Ast = ast :: Adt ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { match self { Adt :: Struct (s) => Some (s . source (db) ? . map (ast :: Adt :: Struct)) , Adt :: Union (u) => Some (u . source (db) ? . map (ast :: Adt :: Union)) , Adt :: Enum (e) => Some (e . source (db) ? . map (ast :: Adt :: Enum)) , } } }
};
}
