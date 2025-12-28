macro_rules! deps {
    () => {
        BorrowckInferCtxt!();
        UniversalRegionIndices!();
        RegionCtxt!();
    };
}

macro_rules! impl_499 {
    () => {
        deps!();
        # [extension (trait InferCtxtExt <'tcx >)] impl < 'tcx > BorrowckInferCtxt < 'tcx > { # [instrument (skip (self) , level = "debug")] fn replace_free_regions_with_nll_infer_vars < T > (& self , origin : NllRegionVariableOrigin , value : T ,) -> T where T : TypeFoldable < TyCtxt < 'tcx > > , { fold_regions (self . infcx . tcx , value , | region , _depth | { let name = region . get_name_or_anon (self . infcx . tcx) ; debug ! (? region , ? name) ; self . next_nll_region_var (origin , | | RegionCtxt :: Free (name)) }) } # [instrument (level = "debug" , skip (self , indices))] fn replace_bound_regions_with_nll_infer_vars < T > (& self , all_outlive_scope : LocalDefId , value : ty :: Binder < 'tcx , T > , indices : & UniversalRegionIndices < 'tcx > ,) -> T where T : TypeFoldable < TyCtxt < 'tcx > > , { let (value , _map) = self . tcx . instantiate_bound_regions (value , | br | { debug ! (? br) ; let kind = ty :: LateParamRegionKind :: from_bound (br . var , br . kind) ; let liberated_region = ty :: Region :: new_late_param (self . tcx , all_outlive_scope . to_def_id () , kind) ; ty :: Region :: new_var (self . tcx , indices . to_region_vid (liberated_region)) }) ; value } }
    };
}

impl_499!()