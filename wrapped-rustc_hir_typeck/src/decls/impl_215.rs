macro_rules! deps {
    () => {
        FnCtxt!();
        LoweredTy!();
    };
}

macro_rules! impl_215 {
    () => {
        deps!();
        impl < 'tcx > LoweredTy < 'tcx > { fn from_raw (fcx : & FnCtxt < '_ , 'tcx > , span : Span , raw : Ty < 'tcx >) -> LoweredTy < 'tcx > { let normalized = if fcx . next_trait_solver () { fcx . try_structurally_resolve_type (span , raw) } else { fcx . normalize (span , raw) } ; LoweredTy { raw , normalized } } }
    };
}

impl_215!();