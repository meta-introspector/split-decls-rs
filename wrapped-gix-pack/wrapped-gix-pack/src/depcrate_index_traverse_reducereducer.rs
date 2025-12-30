// Generated macro for Reducer (struct)
macro_rules! Depcrate_index_traverse_reduceReducer {
() => {
// Module: crate::index::traverse::reduce
// Provides: {"Reducer"}
// Dependencies: {}
pub struct Reducer < 'a , P , E > { progress : OwnShared < Mutable < P > > , check : traverse :: SafetyCheck , then : Instant , entries_seen : usize , stats : traverse :: Statistics , should_interrupt : & 'a AtomicBool , _error : std :: marker :: PhantomData < E > , }
};
}
