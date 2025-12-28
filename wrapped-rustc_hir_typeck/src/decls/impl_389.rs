macro_rules! deps {
    () => {
        EagerlyNormalizeConsts!();
    };
}

macro_rules! impl_389 {
    () => {
        deps!();
        impl < 'tcx > TypeFolder < TyCtxt < 'tcx > > for EagerlyNormalizeConsts < 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . tcx } fn fold_const (& mut self , ct : ty :: Const < 'tcx >) -> ty :: Const < 'tcx > { self . tcx . try_normalize_erasing_regions (self . typing_env , ct) . unwrap_or (ct) } }
    };
}

impl_389!();