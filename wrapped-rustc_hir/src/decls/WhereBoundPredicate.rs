macro_rules! deps {
    () => {
        GenericParam!();
        PredicateOrigin!();
        GenericBounds!();
        Ty!();
    };
}

macro_rules! WhereBoundPredicate {
    () => {
        deps!();
        # [doc = " A type bound (e.g., `for<'c> Foo: Send + Clone + 'c`)."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct WhereBoundPredicate < 'hir > { # [doc = " Origin of the predicate."] pub origin : PredicateOrigin , # [doc = " Any generics from a `for` binding."] pub bound_generic_params : & 'hir [GenericParam < 'hir >] , # [doc = " The type being bounded."] pub bounded_ty : & 'hir Ty < 'hir > , # [doc = " Trait and lifetime bounds (e.g., `Clone + Send + 'static`)."] pub bounds : GenericBounds < 'hir > , }
    };
}

WhereBoundPredicate!()