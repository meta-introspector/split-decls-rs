use serde::{Deserialize, Serialize};
use std::collections::HashMap;
struct CollectRegionConstraintsResult<'tcx> {
    infcx: BorrowckInferCtxt<'tcx>,
    body_owned: Body<'tcx>,
    promoted: IndexVec<Promoted, Body<'tcx>>,
    move_data: MoveData<'tcx>,
    borrow_set: BorrowSet<'tcx>,
    location_table: PoloniusLocationTable,
    location_map: Rc<DenseLocationMap>,
    universal_region_relations: Frozen<UniversalRegionRelations<'tcx>>,
    region_bound_pairs: Frozen<RegionBoundPairs<'tcx>>,
    known_type_outlives_obligations: Frozen<Vec<ty::PolyTypeOutlivesPredicate<'tcx>>>,
    constraints: MirTypeckRegionConstraints<'tcx>,
    deferred_closure_requirements: DeferredClosureRequirements<'tcx>,
    deferred_opaque_type_errors: Vec<DeferredOpaqueTypeError<'tcx>>,
    polonius_facts: Option<AllFacts<RustcFacts>>,
    polonius_context: Option<PoloniusContext>,
}
