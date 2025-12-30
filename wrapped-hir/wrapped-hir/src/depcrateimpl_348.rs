// Generated macro for impl_348 (impl)
macro_rules! Depcrateimpl_348 {
() => {
// Module: crate
// Provides: {"impl_348"}
// Dependencies: {}
impl Struct { pub fn module (self , db : & dyn HirDatabase) -> Module { Module { id : self . id . lookup (db) . container } } pub fn name (self , db : & dyn HirDatabase) -> Name { db . struct_signature (self . id) . name . clone () } pub fn fields (self , db : & dyn HirDatabase) -> Vec < Field > { self . id . fields (db) . fields () . iter () . map (| (id , _) | Field { parent : self . into () , id }) . collect () } pub fn ty (self , db : & dyn HirDatabase) -> Type < '_ > { Type :: from_def (db , self . id) } pub fn ty_params (self , db : & dyn HirDatabase) -> Type < '_ > { Type :: from_def_params (db , self . id) } pub fn constructor_ty (self , db : & dyn HirDatabase) -> Type < '_ > { Type :: from_value_def (db , self . id) } pub fn repr (self , db : & dyn HirDatabase) -> Option < ReprOptions > { db . struct_signature (self . id) . repr } pub fn kind (self , db : & dyn HirDatabase) -> StructKind { match self . variant_fields (db) . shape { hir_def :: item_tree :: FieldsShape :: Record => StructKind :: Record , hir_def :: item_tree :: FieldsShape :: Tuple => StructKind :: Tuple , hir_def :: item_tree :: FieldsShape :: Unit => StructKind :: Unit , } } fn variant_fields (self , db : & dyn HirDatabase) -> & VariantFields { self . id . fields (db) } pub fn is_unstable (self , db : & dyn HirDatabase) -> bool { db . attrs (self . id . into ()) . is_unstable () } pub fn instantiate_infer < 'db > (self , infer_ctxt : & InferCtxt < 'db >) -> InstantiatedStruct < 'db > { let args = infer_ctxt . fresh_args_for_item (self . id . into ()) ; InstantiatedStruct { inner : self , args } } }
};
}
