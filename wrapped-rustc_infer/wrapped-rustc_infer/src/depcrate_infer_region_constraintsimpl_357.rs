// Generated macro for impl_357 (impl)
macro_rules! Depcrate_infer_region_constraintsimpl_357 {
() => {
// Module: crate::infer::region_constraints
// Provides: {"impl_357"}
// Dependencies: {}
impl < 'tcx > Rollback < UndoLog < 'tcx > > for RegionConstraintStorage < 'tcx > { fn reverse (& mut self , undo : UndoLog < 'tcx >) { match undo { AddVar (vid) => { self . var_infos . pop () . unwrap () ; assert_eq ! (self . var_infos . len () , vid . index ()) ; } AddConstraint (index) => { self . data . constraints . pop () . unwrap () ; assert_eq ! (self . data . constraints . len () , index) ; } AddVerify (index) => { self . data . verifys . pop () ; assert_eq ! (self . data . verifys . len () , index) ; } AddCombination (Glb , ref regions) => { self . glbs . remove (regions) ; } AddCombination (Lub , ref regions) => { self . lubs . remove (regions) ; } } } }
};
}
