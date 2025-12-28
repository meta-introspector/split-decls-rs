macro_rules! deps {
    () => {
        RegionRenumberer!();
        RegionCtxt!();
    };
}

macro_rules! impl_388 {
    () => {
        deps!();
        impl < 'a , 'tcx > RegionRenumberer < 'a , 'tcx > { # [doc = " Replaces all regions appearing in `value` with fresh inference"] # [doc = " variables."] fn renumber_regions < T , F > (& mut self , value : T , region_ctxt_fn : F) -> T where T : TypeFoldable < TyCtxt < 'tcx > > , F : Fn () -> RegionCtxt , { let origin = NllRegionVariableOrigin :: Existential { name : None } ; fold_regions (self . infcx . tcx , value , | _region , _depth | { self . infcx . next_nll_region_var (origin , | | region_ctxt_fn ()) }) } }
    };
}

impl_388!()