// Generated macro for all_predicates_of (function)
macro_rules! Depcrate_tyall_predicates_of {
() => {
// Module: crate::ty
// Provides: {"all_predicates_of"}
// Dependencies: {}
# [doc = " Gets an iterator over all predicates which apply to the given item."] pub fn all_predicates_of (tcx : TyCtxt < '_ > , id : DefId) -> impl Iterator < Item = & (ty :: Clause < '_ > , Span) > { let mut next_id = Some (id) ; iter :: from_fn (move | | { next_id . take () . map (| id | { let preds = tcx . predicates_of (id) ; next_id = preds . parent ; preds . predicates . iter () }) }) . flatten () }
};
}
