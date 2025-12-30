// Generated macro for get_class_data (function)
macro_rules! Depcrate_rust_typeget_class_data {
() => {
// Module: crate::rust_type
// Provides: {"get_class_data"}
// Dependencies: {}
fn get_class_data (entity_ref : & Entity < '_ > , context : & Context < '_ > ,) -> (ItemIdentifier , ThreadSafety , Vec < ItemIdentifier >) { let entity = entity_ref . get_location () . expect ("class location") . get_entity () . expect ("class entity") ; let mut id = ItemIdentifier :: new (& entity , context) ; match entity . get_kind () { EntityKind :: ObjCInterfaceDecl => { let thread_safety = ThreadSafety :: from_decl (& entity , context) ; let superclasses = parse_superclasses (& entity , context) . into_iter () . map (| (id , _ , _) | id) . collect () ; (id , thread_safety , superclasses) } EntityKind :: ObjCClassRef => { let thread_safety = ThreadSafety :: from_ref (& entity , context) ; (id , thread_safety , vec ! []) } EntityKind :: MacroExpansion => { id . name = entity_ref . get_name () . unwrap_or_else (| | { error ! (? entity_ref , ? entity , "macro ref did not have name") ; id . name }) ; let thread_safety = ThreadSafety :: dummy () ; let superclasses = vec ! [] ; (id , thread_safety , superclasses) } _ => { error ! (? entity , "was not a class") ; (id , ThreadSafety :: dummy () , vec ! []) } } }
};
}
