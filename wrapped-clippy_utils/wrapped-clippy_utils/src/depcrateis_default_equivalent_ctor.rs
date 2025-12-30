// Generated macro for is_default_equivalent_ctor (function)
macro_rules! Depcrateis_default_equivalent_ctor {
() => {
// Module: crate
// Provides: {"is_default_equivalent_ctor"}
// Dependencies: {}
# [doc = " Returns true if the `def_id` associated with the `path` is recognized as a \"default-equivalent\""] # [doc = " constructor from the std library"] fn is_default_equivalent_ctor (cx : & LateContext < '_ > , def_id : DefId , path : & QPath < '_ >) -> bool { let std_types_symbols = & [sym :: Vec , sym :: VecDeque , sym :: LinkedList , sym :: HashMap , sym :: BTreeMap , sym :: HashSet , sym :: BTreeSet , sym :: BinaryHeap ,] ; if let QPath :: TypeRelative (_ , method) = path && method . ident . name == sym :: new && let Some (impl_did) = cx . tcx . impl_of_assoc (def_id) && let Some (adt) = cx . tcx . type_of (impl_did) . instantiate_identity () . ty_adt_def () { return Some (adt . did ()) == cx . tcx . lang_items () . string () || (cx . tcx . get_diagnostic_name (adt . did ())) . is_some_and (| adt_name | std_types_symbols . contains (& adt_name)) ; } false }
};
}
