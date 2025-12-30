// Generated macro for impl_63 (impl)
macro_rules! Depcrate_has_sourceimpl_63 {
() => {
// Module: crate::has_source
// Provides: {"impl_63"}
// Dependencies: {}
impl HasSource for Field { type Ast = FieldSource ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { let var = VariantId :: from (self . parent) ; let src = var . child_source (db) ; let field_source = src . map (| it | match it [self . id] . clone () { Either :: Left (it) => FieldSource :: Pos (it) , Either :: Right (it) => FieldSource :: Named (it) , }) ; Some (field_source) } }
};
}
