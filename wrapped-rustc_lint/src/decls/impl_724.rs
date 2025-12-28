macro_rules! deps {
    () => {
        LateContext!();
        PassByValueDiag!();
    };
}

macro_rules! impl_724 {
    () => {
        deps!();
        impl < 'tcx > LateLintPass < 'tcx > for PassByValue { fn check_ty (& mut self , cx : & LateContext < '_ > , ty : & 'tcx hir :: Ty < 'tcx , AmbigArg >) { match & ty . kind { TyKind :: Ref (_ , hir :: MutTy { ty : inner_ty , mutbl : hir :: Mutability :: Not }) => { if cx . tcx . trait_impl_of_assoc (ty . hir_id . owner . to_def_id ()) . is_some () { return ; } if let Some (t) = path_for_pass_by_value (cx , inner_ty) { cx . emit_span_lint (PASS_BY_VALUE , ty . span , PassByValueDiag { ty : t , suggestion : ty . span } ,) ; } } _ => { } } } }
    };
}

impl_724!();