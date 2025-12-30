// Generated macro for BorrowSet (struct)
macro_rules! Depcrate_borrow_setBorrowSet {
() => {
// Module: crate::borrow_set
// Provides: {"BorrowSet"}
// Dependencies: {}
pub struct BorrowSet < 'tcx > { # [doc = " The fundamental map relating bitvector indexes to the borrows"] # [doc = " in the MIR. Each borrow is also uniquely identified in the MIR"] # [doc = " by the `Location` of the assignment statement in which it"] # [doc = " appears on the right hand side. Thus the location is the map"] # [doc = " key, and its position in the map corresponds to `BorrowIndex`."] pub (crate) location_map : FxIndexMap < Location , BorrowData < 'tcx > > , # [doc = " Locations which activate borrows."] # [doc = " NOTE: a given location may activate more than one borrow in the future"] # [doc = " when more general two-phase borrow support is introduced, but for now we"] # [doc = " only need to store one borrow index."] pub (crate) activation_map : FxIndexMap < Location , Vec < BorrowIndex > > , # [doc = " Map from local to all the borrows on that local."] pub (crate) local_map : FxIndexMap < mir :: Local , FxIndexSet < BorrowIndex > > , pub (crate) locals_state_at_exit : LocalsStateAtExit , }
};
}
