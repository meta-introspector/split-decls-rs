// Generated macro for GatherBorrows (struct)
macro_rules! Depcrate_borrow_setGatherBorrows {
() => {
// Module: crate::borrow_set
// Provides: {"GatherBorrows"}
// Dependencies: {}
struct GatherBorrows < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , body : & 'a Body < 'tcx > , location_map : FxIndexMap < Location , BorrowData < 'tcx > > , activation_map : FxIndexMap < Location , Vec < BorrowIndex > > , local_map : FxIndexMap < mir :: Local , FxIndexSet < BorrowIndex > > , # [doc = " When we encounter a 2-phase borrow statement, it will always"] # [doc = " be assigning into a temporary TEMP:"] # [doc = ""] # [doc = "    TEMP = &foo"] # [doc = ""] # [doc = " We add TEMP into this map with `b`, where `b` is the index of"] # [doc = " the borrow. When we find a later use of this activation, we"] # [doc = " remove from the map (and add to the \"tombstone\" set below)."] pending_activations : FxIndexMap < mir :: Local , BorrowIndex > , locals_state_at_exit : LocalsStateAtExit , }
};
}
