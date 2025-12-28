macro_rules! deps {
    () => {
        Ty!();
    };
}

macro_rules! MutTy {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct MutTy < 'hir > { pub ty : & 'hir Ty < 'hir > , pub mutbl : Mutability , }
    };
}

MutTy!()