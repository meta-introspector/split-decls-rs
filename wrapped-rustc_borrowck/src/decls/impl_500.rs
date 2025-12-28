macro_rules! deps {
    () => {
        MirTypeckRegionConstraints!();
        UniversalRegionIndices!();
    };
}

macro_rules! impl_500 {
    () => {
        deps!();
        impl < 'tcx > UniversalRegionIndices < 'tcx > { # [doc = " Initially, the `UniversalRegionIndices` map contains only the"] # [doc = " early-bound regions in scope. Once that is all setup, we come"] # [doc = " in later and instantiate the late-bound regions, and then we"] # [doc = " insert the `ReLateParam` version of those into the map as"] # [doc = " well. These are used for error reporting."] fn insert_late_bound_region (& mut self , r : ty :: Region < 'tcx > , vid : ty :: RegionVid) { debug ! ("insert_late_bound_region({:?}, {:?})" , r , vid) ; assert_eq ! (self . indices . insert (r , vid) , None) ; } # [doc = " Converts `r` into a local inference variable: `r` can either"] # [doc = " be a `ReVar` (i.e., already a reference to an inference"] # [doc = " variable) or it can be `'static` or some early-bound"] # [doc = " region. This is useful when taking the results from"] # [doc = " type-checking and trait-matching, which may sometimes"] # [doc = " reference those regions from the `ParamEnv`. It is also used"] # [doc = " during initialization. Relies on the `indices` map having been"] # [doc = " fully initialized."] # [doc = ""] # [doc = " Panics if `r` is not a registered universal region, most notably"] # [doc = " if it is a placeholder. Handling placeholders requires access to the"] # [doc = " `MirTypeckRegionConstraints`."] fn to_region_vid (& self , r : ty :: Region < 'tcx >) -> RegionVid { match r . kind () { ty :: ReVar (..) => r . as_var () , ty :: ReError (guar) => { self . encountered_re_error . set (Some (guar)) ; self . fr_static } _ => * self . indices . get (& r) . unwrap_or_else (| | bug ! ("cannot convert `{:?}` to a region vid" , r)) , } } # [doc = " Replaces all free regions in `value` with region vids, as"] # [doc = " returned by `to_region_vid`."] fn fold_to_region_vids < T > (& self , tcx : TyCtxt < 'tcx > , value : T) -> T where T : TypeFoldable < TyCtxt < 'tcx > > , { fold_regions (tcx , value , | region , _ | ty :: Region :: new_var (tcx , self . to_region_vid (region))) } }
    };
}

impl_500!()