macro_rules! deps {
    () => {
        PoloniusLivenessContext!();
        VarianceExtractor!();
        UniversalRegions!();
    };
}

macro_rules! impl_281 {
    () => {
        deps!();
        impl PoloniusLivenessContext { # [doc = " Record the variance of each region contained within the given value."] pub (crate) fn record_live_region_variance < 'tcx > (& mut self , tcx : TyCtxt < 'tcx > , universal_regions : & UniversalRegions < 'tcx > , value : impl TypeVisitable < TyCtxt < 'tcx > > + Relate < TyCtxt < 'tcx > > ,) { let mut extractor = VarianceExtractor { tcx , ambient_variance : ty :: Variance :: Covariant , directions : & mut self . live_region_variances , universal_regions , } ; extractor . relate (value , value) . expect ("Can't have a type error relating to itself") ; } }
    };
}

impl_281!();