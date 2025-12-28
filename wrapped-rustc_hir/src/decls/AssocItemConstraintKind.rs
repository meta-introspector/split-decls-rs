macro_rules! deps {
    () => {
        Term!();
        Ty!();
        GenericBound!();
    };
}

macro_rules! AssocItemConstraintKind {
    () => {
        deps!();
        # [doc = " The kind of [associated item constraint][AssocItemConstraint]."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub enum AssocItemConstraintKind < 'hir > { # [doc = " An equality constraint for an associated item (e.g., `AssocTy = Ty` in `Trait<AssocTy = Ty>`)."] # [doc = ""] # [doc = " Also known as an *associated item binding* (we *bind* an associated item to a term)."] # [doc = ""] # [doc = " Furthermore, associated type equality constraints can also be referred to as *associated type"] # [doc = " bindings*. Similarly with associated const equality constraints and *associated const bindings*."] Equality { term : Term < 'hir > } , # [doc = " A bound on an associated type (e.g., `AssocTy: Bound` in `Trait<AssocTy: Bound>`)."] Bound { bounds : & 'hir [GenericBound < 'hir >] } , }
    };
}

AssocItemConstraintKind!();