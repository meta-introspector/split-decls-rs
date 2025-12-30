// Generated macro for impl_11263 (impl)
macro_rules! Depcrate_upper_case_acronymsimpl_11263 {
() => {
// Module: crate::upper_case_acronyms
// Provides: {"impl_11263"}
// Dependencies: {}
impl LateLintPass < '_ > for UpperCaseAcronyms { fn check_item (& mut self , cx : & LateContext < '_ > , it : & Item < '_ >) { if it . span . in_external_macro (cx . sess () . source_map ()) || (self . avoid_breaking_exported_api && cx . effective_visibilities . is_exported (it . owner_id . def_id)) { return ; } match it . kind { ItemKind :: TyAlias (ident , ..) | ItemKind :: Struct (ident , ..) | ItemKind :: Trait (_ , _ , _ , ident , ..) => { check_ident (cx , & ident , it . hir_id () , self . upper_case_acronyms_aggressive) ; } , ItemKind :: Enum (ident , _ , ref enumdef) => { check_ident (cx , & ident , it . hir_id () , self . upper_case_acronyms_aggressive) ; enumdef . variants . iter () . for_each (| variant | { check_ident (cx , & variant . ident , variant . hir_id , self . upper_case_acronyms_aggressive) ; }) ; } , _ => { } , } } }
};
}
