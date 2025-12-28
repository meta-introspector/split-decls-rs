macro_rules! explicit_outlives_bounds {
    () => {
        # [instrument (level = "debug" , skip (param_env) , ret)] pub fn explicit_outlives_bounds < 'tcx > (param_env : ty :: ParamEnv < 'tcx > ,) -> impl Iterator < Item = OutlivesBound < 'tcx > > { param_env . caller_bounds () . into_iter () . filter_map (ty :: Clause :: as_region_outlives_clause) . filter_map (ty :: Binder :: no_bound_vars) . map (| ty :: OutlivesPredicate (r_a , r_b) | OutlivesBound :: RegionSubRegion (r_b , r_a)) }
    };
}

explicit_outlives_bounds!();