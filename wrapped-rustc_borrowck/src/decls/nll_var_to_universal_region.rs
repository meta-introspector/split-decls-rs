macro_rules! deps {
    () => {
        RegionCtxt!();
        RegionClassification!();
    };
}

macro_rules! nll_var_to_universal_region {
    () => {
        deps!();
        # [doc = " Maps an NLL var to a deterministically chosen equal universal region."] # [doc = ""] # [doc = " See the corresponding [rustc-dev-guide chapter] for more details. This"] # [doc = " ignores changes to the region values due to member constraints. Applying"] # [doc = " member constraints does not impact the result of this function."] # [doc = ""] # [doc = " [rustc-dev-guide chapter]: https://rustc-dev-guide.rust-lang.org/borrow_check/opaque-types-region-inference-restrictions.html"] fn nll_var_to_universal_region < 'tcx > (rcx : & RegionCtxt < '_ , 'tcx > , r : RegionVid ,) -> Option < Region < 'tcx > > { let vid = rcx . representative (r) . rvid () ; match rcx . definitions [vid] . origin { NllRegionVariableOrigin :: FreeRegion => rcx . universal_regions () . universal_regions_iter () . filter (| & ur | { ! matches ! (rcx . universal_regions () . region_classification (ur) , Some (RegionClassification :: External)) }) . find (| & ur | rcx . universal_region_relations . equal (vid , ur)) . map (| ur | rcx . definitions [ur] . external_name . unwrap ()) , NllRegionVariableOrigin :: Placeholder (placeholder) => { Some (ty :: Region :: new_placeholder (rcx . infcx . tcx , placeholder)) } NllRegionVariableOrigin :: Existential { .. } => None , } }
    };
}

nll_var_to_universal_region!();