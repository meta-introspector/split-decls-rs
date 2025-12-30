// Generated macro for fn_eagerness (function)
macro_rules! Depcrate_eager_or_lazyfn_eagerness {
() => {
// Module: crate::eager_or_lazy
// Provides: {"fn_eagerness"}
// Dependencies: {}
# [doc = " Determine the eagerness of the given function call."] fn fn_eagerness (cx : & LateContext < '_ > , fn_id : DefId , name : Symbol , have_one_arg : bool) -> EagernessSuggestion { use EagernessSuggestion :: { Eager , Lazy , NoChange } ; let ty = match cx . tcx . impl_of_assoc (fn_id) { Some (id) => cx . tcx . type_of (id) . instantiate_identity () , None => return Lazy , } ; if (matches ! (name , sym :: is_empty | sym :: len) || name . as_str () . starts_with ("as_")) && have_one_arg { if matches ! (cx . tcx . crate_name (fn_id . krate) , sym :: std | sym :: core | sym :: alloc | sym :: proc_macro) { Eager } else { NoChange } } else if let ty :: Adt (def , subs) = ty . kind () { if def . variants () . iter () . flat_map (| v | v . fields . iter ()) . any (| x | { matches ! (cx . tcx . type_of (x . did) . instantiate_identity () . peel_refs () . kind () , ty :: Param (_)) }) && all_predicates_of (cx . tcx , fn_id) . all (| (pred , _) | match pred . kind () . skip_binder () { ty :: ClauseKind :: Trait (pred) => cx . tcx . trait_def (pred . trait_ref . def_id) . is_marker , _ => true , }) && subs . types () . all (| x | matches ! (x . peel_refs () . kind () , ty :: Param (_))) { match & * * cx . tcx . fn_sig (fn_id) . instantiate_identity () . skip_binder () . inputs_and_output { [arg , res] if ! arg . is_mutable_ptr () && arg . peel_refs () == ty && res . is_bool () => NoChange , _ => Lazy , } } else { Lazy } } else { Lazy } }
};
}
