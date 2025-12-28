macro_rules! deps {
    () => {
        GenericParam!();
        Ty!();
    };
}

macro_rules! UnsafeBinderTy {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct UnsafeBinderTy < 'hir > { pub generic_params : & 'hir [GenericParam < 'hir >] , pub inner_ty : & 'hir Ty < 'hir > , }
    };
}

UnsafeBinderTy!()