// Generated macro for Seen (struct)
macro_rules! Depcrate_dfa_sparseSeen {
() => {
// Module: crate::dfa::sparse
// Provides: {"Seen"}
// Dependencies: {}
# [derive (Debug)] struct Seen { # [cfg (feature = "alloc")] set : alloc :: collections :: BTreeSet < StateID > , # [cfg (not (feature = "alloc"))] set : core :: marker :: PhantomData < StateID > , }
};
}
