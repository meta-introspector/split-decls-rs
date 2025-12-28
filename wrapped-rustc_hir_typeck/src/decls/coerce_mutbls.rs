macro_rules! coerce_mutbls {
    () => {
        # [doc = " Coercing a mutable reference to an immutable works, while"] # [doc = " coercing `&T` to `&mut T` should be forbidden."] fn coerce_mutbls < 'tcx > (from_mutbl : hir :: Mutability , to_mutbl : hir :: Mutability ,) -> RelateResult < 'tcx , () > { if from_mutbl >= to_mutbl { Ok (()) } else { Err (TypeError :: Mutability) } }
    };
}

coerce_mutbls!();