macro_rules! deps {
    () => {
        UniversalRegions!();
        BorrowckInferCtxt!();
        MirTypeckRegionConstraints!();
        RegionValues!();
        UniversalRegionRelations!();
        Representative!();
        OutlivesConstraintSet!();
        SccAnnotations!();
        RegionTracker!();
        RegionCtxt!();
        ReverseSccGraph!();
        ConstraintSccs!();
    };
}

macro_rules! impl_334 {
    () => {
        deps!();
        impl < 'a , 'tcx > RegionCtxt < 'a , 'tcx > { # [doc = " Creates a new `RegionCtxt` used to compute defining opaque type uses."] # [doc = ""] # [doc = " This does not yet propagate region values. This is instead done lazily"] # [doc = " when applying member constraints."] pub (super) fn new (infcx : & 'a BorrowckInferCtxt < 'tcx > , universal_region_relations : & 'a Frozen < UniversalRegionRelations < 'tcx > > , location_map : Rc < DenseLocationMap > , constraints : & MirTypeckRegionConstraints < 'tcx > ,) -> RegionCtxt < 'a , 'tcx > { let mut outlives_constraints = constraints . outlives_constraints . clone () ; let universal_regions = & universal_region_relations . universal_regions ; let (definitions , _has_placeholders) = region_definitions (infcx , universal_regions) ; let compute_sccs = | outlives_constraints : & OutlivesConstraintSet < 'tcx > , annotations : & mut SccAnnotations < '_ , 'tcx , RegionTracker > | { ConstraintSccs :: new_with_annotation (& outlives_constraints . graph (definitions . len ()) . region_graph (outlives_constraints , universal_regions . fr_static) , annotations ,) } ; let mut scc_annotations = SccAnnotations :: init (& definitions) ; let mut constraint_sccs = compute_sccs (& outlives_constraints , & mut scc_annotations) ; let added_constraints = crate :: handle_placeholders :: rewrite_placeholder_outlives (& constraint_sccs , & scc_annotations , universal_regions . fr_static , & mut outlives_constraints ,) ; if added_constraints { scc_annotations = SccAnnotations :: init (& definitions) ; constraint_sccs = compute_sccs (& outlives_constraints , & mut scc_annotations) ; } let scc_annotations = scc_annotations . scc_to_annotation ; let placeholder_indices = Default :: default () ; let mut scc_values = RegionValues :: new (location_map , universal_regions . len () , placeholder_indices) ; for variable in definitions . indices () { let scc = constraint_sccs . scc (variable) ; match definitions [variable] . origin { NllRegionVariableOrigin :: FreeRegion => { scc_values . add_element (scc , variable) ; } _ => { } } } let rev_scc_graph = ReverseSccGraph :: compute (& constraint_sccs , universal_regions) ; RegionCtxt { infcx , definitions , universal_region_relations , constraint_sccs , scc_annotations , rev_scc_graph , scc_values , } } pub (super) fn representative (& self , vid : RegionVid) -> Representative { let scc = self . constraint_sccs . scc (vid) ; self . scc_annotations [scc] . representative } pub (crate) fn max_placeholder_universe_reached (& self , scc : ConstraintSccIndex ,) -> UniverseIndex { self . scc_annotations [scc] . max_placeholder_universe_reached () } pub (super) fn universal_regions (& self) -> & UniversalRegions < 'tcx > { & self . universal_region_relations . universal_regions } pub (super) fn eval_equal (& self , r1_vid : RegionVid , r2_vid : RegionVid) -> bool { let r1 = self . constraint_sccs . scc (r1_vid) ; let r2 = self . constraint_sccs . scc (r2_vid) ; if r1 == r2 { return true ; } let universal_outlives = | sub , sup | { self . scc_values . universal_regions_outlived_by (sub) . all (| r1 | { self . scc_values . universal_regions_outlived_by (sup) . any (| r2 | self . universal_region_relations . outlives (r2 , r1)) }) } ; universal_outlives (r1 , r2) && universal_outlives (r2 , r1) } }
    };
}

impl_334!();