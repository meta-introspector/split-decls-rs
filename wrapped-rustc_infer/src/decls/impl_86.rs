macro_rules! deps {
    () => {
        OutlivesEnvironment!();
        FreeRegionMap!();
        GenericKind!();
        RegionBoundPairs!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < 'tcx > OutlivesEnvironment < 'tcx > { # [doc = " Create a new `OutlivesEnvironment` from normalized outlives bounds."] pub fn from_normalized_bounds (param_env : ty :: ParamEnv < 'tcx > , known_type_outlives : Vec < ty :: PolyTypeOutlivesPredicate < 'tcx > > , extra_bounds : impl IntoIterator < Item = OutlivesBound < 'tcx > > , higher_ranked_assumptions : FxHashSet < ty :: ArgOutlivesPredicate < 'tcx > > ,) -> Self { let mut region_relation = TransitiveRelationBuilder :: default () ; let mut region_bound_pairs = RegionBoundPairs :: default () ; for outlives_bound in explicit_outlives_bounds (param_env) . chain (extra_bounds) { debug ! ("add_outlives_bounds: outlives_bound={:?}" , outlives_bound) ; match outlives_bound { OutlivesBound :: RegionSubParam (r_a , param_b) => { region_bound_pairs . insert (ty :: OutlivesPredicate (GenericKind :: Param (param_b) , r_a)) ; } OutlivesBound :: RegionSubAlias (r_a , alias_b) => { region_bound_pairs . insert (ty :: OutlivesPredicate (GenericKind :: Alias (alias_b) , r_a)) ; } OutlivesBound :: RegionSubRegion (r_a , r_b) => match (r_a . kind () , r_b . kind ()) { (ty :: ReStatic | ty :: ReEarlyParam (_) | ty :: ReLateParam (_) , ty :: ReStatic | ty :: ReEarlyParam (_) | ty :: ReLateParam (_) ,) => region_relation . add (r_a , r_b) , (ty :: ReError (_) , _) | (_ , ty :: ReError (_)) => { } (ty :: ReVar (_) , _) | (_ , ty :: ReVar (_)) => { } _ => bug ! ("add_outlives_bounds: unexpected regions: ({r_a:?}, {r_b:?})") , } , } } OutlivesEnvironment { param_env , known_type_outlives , free_region_map : FreeRegionMap { relation : region_relation . freeze () } , region_bound_pairs , higher_ranked_assumptions , } } pub fn free_region_map (& self) -> & FreeRegionMap < 'tcx > { & self . free_region_map } pub fn region_bound_pairs (& self) -> & RegionBoundPairs < 'tcx > { & self . region_bound_pairs } pub fn known_type_outlives (& self) -> & [ty :: PolyTypeOutlivesPredicate < 'tcx >] { & self . known_type_outlives } pub fn higher_ranked_assumptions (& self) -> & FxHashSet < ty :: ArgOutlivesPredicate < 'tcx > > { & self . higher_ranked_assumptions } }
    };
}

impl_86!()