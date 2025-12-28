macro_rules! deps {
    () => {
        RegionInferenceContext!();
        ReverseSccGraph!();
    };
}

macro_rules! impl_350 {
    () => {
        deps!();
        impl < 'tcx > RegionInferenceContext < 'tcx > { # [doc = " Map the regions in the type to named regions. This is similar to what"] # [doc = " `infer_opaque_types` does, but can infer any universal region, not only"] # [doc = " ones from the args for the opaque type. It also doesn't double check"] # [doc = " that the regions produced are in fact equal to the named region they are"] # [doc = " replaced with. This is fine because this function is only to improve the"] # [doc = " region names in error messages."] # [doc = ""] # [doc = " This differs from `MirBorrowckCtxt::name_regions` since it is particularly"] # [doc = " lax with mapping region vids that are *shorter* than a universal region to"] # [doc = " that universal region. This is useful for member region constraints since"] # [doc = " we want to suggest a universal region name to capture even if it's technically"] # [doc = " not equal to the error region."] pub (crate) fn name_regions_for_member_constraint < T > (& self , tcx : TyCtxt < 'tcx > , ty : T) -> T where T : TypeFoldable < TyCtxt < 'tcx > > , { fold_regions (tcx , ty , | region , _ | match region . kind () { ty :: ReVar (vid) => { let scc = self . constraint_sccs . scc (vid) ; if ! self . max_nameable_universe (scc) . is_root () { match self . scc_values . placeholders_contained_in (scc) . enumerate () . last () { Some ((0 , placeholder)) => { return ty :: Region :: new_placeholder (tcx , placeholder) ; } _ => return region , } } let upper_bound = self . approx_universal_upper_bound (vid) ; if let Some (universal_region) = self . definitions [upper_bound] . external_name { return universal_region ; } let scc = self . constraint_sccs . scc (vid) ; let rev_scc_graph = ReverseSccGraph :: compute (& self . constraint_sccs , self . universal_regions ()) ; let upper_bounds : Vec < _ > = rev_scc_graph . upper_bounds (scc) . filter_map (| vid | self . definitions [vid] . external_name) . filter (| r | ! r . is_static ()) . collect () ; match & upper_bounds [..] { [universal_region] => * universal_region , _ => region , } } _ => region , }) } }
    };
}

impl_350!();