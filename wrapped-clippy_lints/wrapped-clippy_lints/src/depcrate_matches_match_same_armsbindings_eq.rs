// Generated macro for bindings_eq (function)
macro_rules! Depcrate_matches_match_same_armsbindings_eq {
() => {
// Module: crate::matches::match_same_arms
// Provides: {"bindings_eq"}
// Dependencies: {}
# [doc = " Returns true if all the bindings in the `Pat` are in `ids` and vice versa"] fn bindings_eq (pat : & Pat < '_ > , mut ids : HirIdSet) -> bool { let mut result = true ; pat . each_binding_or_first (& mut | _ , id , _ , _ | result &= ids . swap_remove (& id)) ; result && ids . is_empty () }
};
}
