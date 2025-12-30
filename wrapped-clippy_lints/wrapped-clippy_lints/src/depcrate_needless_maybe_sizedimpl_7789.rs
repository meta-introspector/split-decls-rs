// Generated macro for impl_7789 (impl)
macro_rules! Depcrate_needless_maybe_sizedimpl_7789 {
() => {
// Module: crate::needless_maybe_sized
// Provides: {"impl_7789"}
// Dependencies: {}
impl LateLintPass < '_ > for NeedlessMaybeSized { fn check_generics (& mut self , cx : & LateContext < '_ > , generics : & Generics < '_ >) { let Some (sized_trait) = cx . tcx . lang_items () . sized_trait () else { return ; } ; let maybe_sized_params : DefIdMap < _ > = type_param_bounds (generics) . filter (| bound | { bound . trait_bound . trait_ref . trait_def_id () == Some (sized_trait) && matches ! (bound . trait_bound . modifiers . polarity , BoundPolarity :: Maybe (_)) }) . map (| bound | (bound . param , bound)) . collect () ; for bound in type_param_bounds (generics) { if bound . trait_bound . modifiers == TraitBoundModifiers :: NONE && let Some (sized_bound) = maybe_sized_params . get (& bound . param) && let Some (path) = path_to_sized_bound (cx , bound . trait_bound) { span_lint_and_then (cx , NEEDLESS_MAYBE_SIZED , sized_bound . trait_bound . span , "`?Sized` bound is ignored because of a `Sized` requirement" , | diag | { let ty_param = sized_bound . ident ; diag . span_note (bound . trait_bound . span , format ! ("`{ty_param}` cannot be unsized because of the bound") ,) ; for & [current_id , next_id] in path . array_windows () { let current = cx . tcx . item_name (current_id) ; let next = cx . tcx . item_name (next_id) ; diag . note (format ! ("...because `{current}` has the bound `{next}`")) ; } diag . span_suggestion_verbose (generics . span_for_bound_removal (sized_bound . predicate_pos , sized_bound . bound_pos) , "change the bounds that require `Sized`, or remove the `?Sized` bound" , "" , Applicability :: MaybeIncorrect ,) ; } ,) ; return ; } } } }
};
}
