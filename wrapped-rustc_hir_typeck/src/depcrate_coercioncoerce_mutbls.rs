// Generated macro for coerce_mutbls (function)
macro_rules! Depcrate_coercioncoerce_mutbls {
() => {
// Module: crate::coercion
// Provides: {"coerce_mutbls"}
// Dependencies: {}
# [doc = " Coercing a mutable reference to an immutable works, while"] # [doc = " coercing `&T` to `&mut T` should be forbidden."] fn coerce_mutbls < 'tcx > (from_mutbl : hir :: Mutability , to_mutbl : hir :: Mutability ,) -> RelateResult < 'tcx , () > { if from_mutbl >= to_mutbl { Ok (()) } else { Err (TypeError :: Mutability) } }
};
}
