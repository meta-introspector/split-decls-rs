// Generated macro for impl_342 (impl)
macro_rules! Depcrate_index_traverse_reduceimpl_342 {
() => {
// Module: crate::index::traverse::reduce
// Provides: {"impl_342"}
// Dependencies: {}
impl < 'a , P , E > Reducer < 'a , P , E > where P : Progress , { pub fn from_progress (progress : OwnShared < Mutable < P > > , pack_data_len_in_bytes : usize , check : traverse :: SafetyCheck , should_interrupt : & 'a AtomicBool ,) -> Self { let stats = traverse :: Statistics { pack_size : pack_data_len_in_bytes as u64 , .. Default :: default () } ; Reducer { progress , check , then : Instant :: now () , entries_seen : 0 , should_interrupt , stats , _error : Default :: default () , } } }
};
}
