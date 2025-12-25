use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn get_flow_results<'a, 'tcx>(
    tcx: TyCtxt<'tcx>,
    body: &'a Body<'tcx>,
    move_data: &'a MoveData<'tcx>,
    borrow_set: &'a BorrowSet<'tcx>,
    regioncx: &RegionInferenceContext<'tcx>,
) -> Results<'tcx, Borrowck<'a, 'tcx>> {
    let borrows = Borrows::new(tcx, body, regioncx, borrow_set).iterate_to_fixpoint(
        tcx,
        body,
        Some("borrowck"),
    );
    let uninits = MaybeUninitializedPlaces::new(tcx, body, move_data).iterate_to_fixpoint(
        tcx,
        body,
        Some("borrowck"),
    );
    let ever_inits = EverInitializedPlaces::new(body, move_data).iterate_to_fixpoint(
        tcx,
        body,
        Some("borrowck"),
    );
    let analysis = Borrowck {
        borrows: borrows.analysis,
        uninits: uninits.analysis,
        ever_inits: ever_inits.analysis,
    };
    assert_eq!(borrows.entry_states.len(), uninits.entry_states.len());
    assert_eq!(borrows.entry_states.len(), ever_inits.entry_states.len());
    let entry_states: EntryStates<_> = itertools::izip!(
        borrows.entry_states,
        uninits.entry_states,
        ever_inits.entry_states
    )
    .map(|(borrows, uninits, ever_inits)| BorrowckDomain {
        borrows,
        uninits,
        ever_inits,
    })
    .collect();
    Results {
        analysis,
        entry_states,
    }
}
