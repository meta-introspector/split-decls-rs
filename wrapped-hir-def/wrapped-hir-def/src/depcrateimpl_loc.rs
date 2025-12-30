// Generated macro for impl_loc (macro)
macro_rules! Depcrateimpl_loc {
() => {
// Module: crate
// Provides: {"impl_loc"}
// Dependencies: {}
macro_rules ! impl_loc { ($ loc : ident , $ id : ident : $ id_ty : ident , $ container : ident : $ container_type : ident) => { impl AstIdLoc for $ loc { type Container = $ container_type ; type Ast = ast ::$ id_ty ; fn ast_id (& self) -> AstId < Self :: Ast > { self .$ id } fn container (& self) -> Self :: Container { self .$ container } } impl HasModule for $ loc { # [inline] fn module (& self , db : & dyn DefDatabase) -> ModuleId { self .$ container . module (db) } } } ; }
};
}
