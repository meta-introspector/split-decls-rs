macro_rules! deps {
    () => {
        TraitEnvironment!();
    };
}

macro_rules! structurally_normalize_ty {
    () => {
        deps!();
        # [doc = " This should be used in `hir` only."] pub fn structurally_normalize_ty < 'db > (infcx : & InferCtxt < 'db > , ty : Ty < 'db > , env : Arc < TraitEnvironment < 'db > > ,) -> Ty < 'db > { let TyKind :: Alias (..) = ty . kind () else { return ty } ; let mut ocx = ObligationCtxt :: new (infcx) ; let ty = ocx . structurally_normalize_ty (& ObligationCause :: dummy () , env . env , ty) . unwrap_or (ty) ; ty . replace_infer_with_error (infcx . interner) }
    };
}

structurally_normalize_ty!();