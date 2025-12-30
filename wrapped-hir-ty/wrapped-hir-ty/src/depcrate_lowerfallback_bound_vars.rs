// Generated macro for fallback_bound_vars (function)
macro_rules! Depcrate_lowerfallback_bound_vars {
() => {
// Module: crate::lower
// Provides: {"fallback_bound_vars"}
// Dependencies: {}
# [doc = " Replaces any 'free' `BoundVar`s in `s` by `TyKind::Error` from the perspective of generic"] # [doc = " parameter whose index is `param_index`. A `BoundVar` is free when it appears after the"] # [doc = " generic parameter of `param_index`."] fn fallback_bound_vars < T : TypeFoldable < Interner > + HasInterner < Interner = Interner > > (s : T , param_index : usize ,) -> T { let is_allowed = | index | (0 .. param_index) . contains (& index) ; crate :: fold_free_vars (s , | bound , binders | { if bound . index_if_innermost () . is_none_or (is_allowed) { bound . shifted_in_from (binders) . to_ty (Interner) } else { TyKind :: Error . intern (Interner) } } , | ty , bound , binders | { if bound . index_if_innermost () . is_none_or (is_allowed) { bound . shifted_in_from (binders) . to_const (Interner , ty) } else { unknown_const (ty) } } ,) }
};
}
