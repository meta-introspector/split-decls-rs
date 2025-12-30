// Generated macro for impl_443 (impl)
macro_rules! Depcrateimpl_443 {
() => {
// Module: crate
// Provides: {"impl_443"}
// Dependencies: {}
impl BuiltinAttr { pub (crate) fn by_name (db : & dyn HirDatabase , krate : Crate , name : & str) -> Option < Self > { if let builtin @ Some (_) = Self :: builtin (name) { return builtin ; } let idx = crate_def_map (db , krate . id) . registered_attrs () . iter () . position (| it | it . as_str () == name) ? as u32 ; Some (BuiltinAttr { krate : Some (krate . id) , idx }) } fn builtin (name : & str) -> Option < Self > { hir_expand :: inert_attr_macro :: find_builtin_attr_idx (& Symbol :: intern (name)) . map (| idx | BuiltinAttr { krate : None , idx : idx as u32 }) } pub fn name (& self , db : & dyn HirDatabase) -> Name { match self . krate { Some (krate) => Name :: new_symbol_root (crate_def_map (db , krate) . registered_attrs () [self . idx as usize] . clone () ,) , None => Name :: new_symbol_root (Symbol :: intern (hir_expand :: inert_attr_macro :: INERT_ATTRIBUTES [self . idx as usize] . name ,)) , } } pub fn template (& self , _ : & dyn HirDatabase) -> Option < AttributeTemplate > { match self . krate { Some (_) => None , None => { Some (hir_expand :: inert_attr_macro :: INERT_ATTRIBUTES [self . idx as usize] . template) } } } }
};
}
