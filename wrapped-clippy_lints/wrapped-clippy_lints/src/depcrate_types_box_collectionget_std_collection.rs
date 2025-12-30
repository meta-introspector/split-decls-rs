// Generated macro for get_std_collection (function)
macro_rules! Depcrate_types_box_collectionget_std_collection {
() => {
// Module: crate::types::box_collection
// Provides: {"get_std_collection"}
// Dependencies: {}
fn get_std_collection (cx : & LateContext < '_ > , qpath : & QPath < '_ >) -> Option < Symbol > { let param = qpath_generic_tys (qpath) . next () ? ; let id = param . basic_res () . opt_def_id () ? ; cx . tcx . get_diagnostic_name (id) . filter (| & name | { matches ! (name , sym :: HashMap | sym :: Vec | sym :: HashSet | sym :: VecDeque | sym :: LinkedList | sym :: BTreeMap | sym :: BTreeSet | sym :: BinaryHeap) }) . or_else (| | { cx . tcx . lang_items () . string () . filter (| did | id == * did) . map (| _ | sym :: String) }) }
};
}
