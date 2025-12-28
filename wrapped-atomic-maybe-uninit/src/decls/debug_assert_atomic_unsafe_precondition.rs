macro_rules! debug_assert_atomic_unsafe_precondition {
    () => {
        # [allow (unused_macros)] macro_rules ! debug_assert_atomic_unsafe_precondition { ($ ptr : ident , $ ty : ident) => { { # [cfg (atomic_maybe_uninit_no_strict_provenance)] # [allow (unused_imports)] use crate :: utils :: ptr :: { ConstPtrExt as _ , MutPtrExt as _ } ; # [allow (clippy :: arithmetic_side_effects)] { debug_assert ! ($ ptr . addr () & const_eval ! (=> usize { mem :: size_of ::<$ ty > () - 1 }) == 0) ; } } } ; }
    };
}

debug_assert_atomic_unsafe_precondition!();