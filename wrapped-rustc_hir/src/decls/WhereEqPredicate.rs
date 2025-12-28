macro_rules! deps {
    () => {
        Ty!();
    };
}

macro_rules! WhereEqPredicate {
    () => {
        deps!();
        # [doc = " An equality predicate (e.g., `T = int`); currently unsupported."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct WhereEqPredicate < 'hir > { pub lhs_ty : & 'hir Ty < 'hir > , pub rhs_ty : & 'hir Ty < 'hir > , }
    };
}

WhereEqPredicate!()