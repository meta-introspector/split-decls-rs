// Generated macro for ProjectionStore (struct)
macro_rules! Depcrate_mirProjectionStore {
() => {
// Module: crate::mir
// Provides: {"ProjectionStore"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq)] pub struct ProjectionStore < 'db > { id_to_proj : FxHashMap < ProjectionId , Box < [PlaceElem < 'db >] > > , proj_to_id : FxHashMap < Box < [PlaceElem < 'db >] > , ProjectionId > , }
};
}
