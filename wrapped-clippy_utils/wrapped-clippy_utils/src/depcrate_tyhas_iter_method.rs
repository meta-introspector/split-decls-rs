// Generated macro for has_iter_method (function)
macro_rules! Depcrate_tyhas_iter_method {
() => {
// Module: crate::ty
// Provides: {"has_iter_method"}
// Dependencies: {}
# [doc = " If `ty` is known to have a `iter` or `iter_mut` method, returns a symbol representing the type."] pub fn has_iter_method (cx : & LateContext < '_ > , probably_ref_ty : Ty < '_ >) -> Option < Symbol > { let into_iter_collections : & [Symbol] = & [sym :: Vec , sym :: Option , sym :: Result , sym :: BTreeMap , sym :: BTreeSet , sym :: VecDeque , sym :: LinkedList , sym :: BinaryHeap , sym :: HashSet , sym :: HashMap , sym :: PathBuf , sym :: Path , sym :: Receiver ,] ; let ty_to_check = match probably_ref_ty . kind () { ty :: Ref (_ , ty_to_check , _) => * ty_to_check , _ => probably_ref_ty , } ; let def_id = match ty_to_check . kind () { ty :: Array (..) => return Some (sym :: array) , ty :: Slice (..) => return Some (sym :: slice) , ty :: Adt (adt , _) => adt . did () , _ => return None , } ; for & name in into_iter_collections { if cx . tcx . is_diagnostic_item (name , def_id) { return Some (cx . tcx . item_name (def_id)) ; } } None }
};
}
