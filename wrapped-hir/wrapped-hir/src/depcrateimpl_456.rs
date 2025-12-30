// Generated macro for impl_456 (impl)
macro_rules! Depcrateimpl_456 {
() => {
// Module: crate
// Provides: {"impl_456"}
// Dependencies: {}
impl ConstParam { pub fn merge (self) -> TypeOrConstParam { TypeOrConstParam { id : self . id . into () } } pub fn name (self , db : & dyn HirDatabase) -> Name { let params = db . generic_params (self . id . parent ()) ; match params [self . id . local_id ()] . name () { Some (it) => it . clone () , None => { never ! () ; Name :: missing () } } } pub fn module (self , db : & dyn HirDatabase) -> Module { self . id . parent () . module (db) . into () } pub fn parent (self , _db : & dyn HirDatabase) -> GenericDef { self . id . parent () . into () } pub fn ty (self , db : & dyn HirDatabase) -> Type < '_ > { Type :: new (db , self . id . parent () , db . const_param_ty (self . id)) } pub fn default (self , db : & dyn HirDatabase , display_target : DisplayTarget ,) -> Option < ast :: ConstArg > { let arg = generic_arg_from_param (db , self . id . into ()) ? ; known_const_to_ast (arg . constant (Interner) ? , db , display_target) } }
};
}
