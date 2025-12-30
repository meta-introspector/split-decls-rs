// Generated macro for impl_8049 (impl)
macro_rules! Depcrate_non_expressive_namesimpl_8049 {
() => {
// Module: crate::non_expressive_names
// Provides: {"impl_8049"}
// Dependencies: {}
impl EarlyLintPass for NonExpressiveNames { fn check_item (& mut self , cx : & EarlyContext < '_ > , item : & Item) { if item . span . in_external_macro (cx . sess () . source_map ()) { return ; } if let ItemKind :: Fn (box ast :: Fn { ref sig , body : Some (ref blk) , .. }) = item . kind { do_check (self , cx , & item . attrs , & sig . decl , blk) ; } } fn check_impl_item (& mut self , cx : & EarlyContext < '_ > , item : & AssocItem) { if item . span . in_external_macro (cx . sess () . source_map ()) { return ; } if let AssocItemKind :: Fn (box ast :: Fn { ref sig , body : Some (ref blk) , .. }) = item . kind { do_check (self , cx , & item . attrs , & sig . decl , blk) ; } } }
};
}
