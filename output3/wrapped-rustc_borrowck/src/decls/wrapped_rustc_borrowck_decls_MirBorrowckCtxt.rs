use serde::{Deserialize, Serialize};
use std::collections::HashMap;
struct MirBorrowckCtxt<'a, 'infcx, 'tcx> {
    root_cx: &'a mut BorrowCheckRootCtxt<'tcx>,
    infcx: &'infcx BorrowckInferCtxt<'tcx>,
    body: &'a Body<'tcx>,
    move_data: &'a MoveData<'tcx>,
    /// Map from MIR `Location` to `LocationIndex`; created
    /// when MIR borrowck begins.
    location_table: &'a PoloniusLocationTable,
    movable_coroutine: bool,
    /// This field keeps track of when borrow errors are reported in the access_place function
    /// so that there is no duplicate reporting. This field cannot also be used for the conflicting
    /// borrow errors that is handled by the `reservation_error_reported` field as the inclusion
    /// of the `Span` type (while required to mute some errors) stops the muting of the reservation
    /// errors.
    access_place_error_reported: FxIndexSet<(Place<'tcx>, Span)>,
    /// This field keeps track of when borrow conflict errors are reported
    /// for reservations, so that we don't report seemingly duplicate
    /// errors for corresponding activations.
    reservation_error_reported: FxIndexSet<Place<'tcx>>,
    /// This fields keeps track of the `Span`s that we have
    /// used to report extra information for `FnSelfUse`, to avoid
    /// unnecessarily verbose errors.
    fn_self_span_reported: FxIndexSet<Span>,
    /// This field keeps track of errors reported in the checking of uninitialized variables,
    /// so that we don't report seemingly duplicate errors.
    uninitialized_error_reported: FxIndexSet<Local>,
    /// This field keeps track of all the local variables that are declared mut and are mutated.
    /// Used for the warning issued by an unused mutable local variable.
    used_mut: FxIndexSet<Local>,
    /// If the function we're checking is a closure, then we'll need to report back the list of
    /// mutable upvars that have been used. This field keeps track of them.
    used_mut_upvars: SmallVec<FieldIdx, 8>,
    /// Region inference context. This contains the results from region inference and lets us e.g.
    /// find out which CFG points are contained in each borrow region.
    regioncx: &'a RegionInferenceContext<'tcx>,
    /// The set of borrows extracted from the MIR
    borrow_set: &'a BorrowSet<'tcx>,
    /// Information about upvars not necessarily preserved in types or MIR
    upvars: &'tcx [&'tcx ty::CapturedPlace<'tcx>],
    /// Names of local (user) variables (extracted from `var_debug_info`).
    local_names: OnceCell<IndexVec<Local, Option<Symbol>>>,
    /// Record the region names generated for each region in the given
    /// MIR def so that we can reuse them later in help/error messages.
    region_names: RefCell<FxIndexMap<RegionVid, RegionName>>,
    /// The counter for generating new region names.
    next_region_name: RefCell<usize>,
    diags_buffer: &'a mut BorrowckDiagnosticsBuffer<'infcx, 'tcx>,
    move_errors: Vec<MoveError<'tcx>>,
    /// Results of Polonius analysis.
    polonius_output: Option<&'a PoloniusOutput>,
    /// When using `-Zpolonius=next`: the data used to compute errors and diagnostics.
    polonius_diagnostics: Option<&'a PoloniusDiagnosticsContext>,
}
