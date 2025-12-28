macro_rules! deps {
    () => {
        MirTypeckRegionConstraints!();
    };
}

macro_rules! impl_477 {
    () => {
        deps!();
        impl < 'tcx > MirTypeckRegionConstraints < 'tcx > { # [doc = " Creates a `Region` for a given `PlaceholderRegion`, or returns the"] # [doc = " region that corresponds to a previously created one."] pub (crate) fn placeholder_region (& mut self , infcx : & InferCtxt < 'tcx > , placeholder : ty :: PlaceholderRegion ,) -> ty :: Region < 'tcx > { let placeholder_index = self . placeholder_indices . insert (placeholder) ; match self . placeholder_index_to_region . get (placeholder_index) { Some (& v) => v , None => { let origin = NllRegionVariableOrigin :: Placeholder (placeholder) ; let region = infcx . next_nll_region_var_in_universe (origin , placeholder . universe) ; self . placeholder_index_to_region . push (region) ; region } } } }
    };
}

impl_477!();