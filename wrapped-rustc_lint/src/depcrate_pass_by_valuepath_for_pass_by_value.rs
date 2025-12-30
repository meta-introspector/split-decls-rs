// Generated macro for path_for_pass_by_value (function)
macro_rules! Depcrate_pass_by_valuepath_for_pass_by_value {
() => {
// Module: crate::pass_by_value
// Provides: {"path_for_pass_by_value"}
// Dependencies: {}
fn path_for_pass_by_value (cx : & LateContext < '_ > , ty : & hir :: Ty < '_ >) -> Option < String > { if let TyKind :: Path (QPath :: Resolved (_ , path)) = & ty . kind { match path . res { Res :: Def (_ , def_id) if find_attr ! (cx . tcx . get_all_attrs (def_id) , AttributeKind :: PassByValue (_)) => { let name = cx . tcx . item_ident (def_id) ; let path_segment = path . segments . last () . unwrap () ; return Some (format ! ("{}{}" , name , gen_args (cx , path_segment))) ; } Res :: SelfTyAlias { alias_to : did , is_trait_impl : false , .. } => { if let ty :: Adt (adt , args) = cx . tcx . type_of (did) . instantiate_identity () . kind () { if find_attr ! (cx . tcx . get_all_attrs (adt . did ()) , AttributeKind :: PassByValue (_)) { return Some (cx . tcx . def_path_str_with_args (adt . did () , args)) ; } } } _ => () , } } None }
};
}
