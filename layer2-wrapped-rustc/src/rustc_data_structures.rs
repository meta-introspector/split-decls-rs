// rustc_data_structures module stub

pub mod fx {
    pub struct FxHashMap;
    pub struct FxHashSet;
    pub struct FxIndexMap;
    pub struct FxIndexSet;
}

pub mod sync {
    pub struct DynSend;
    pub struct DynSync;
    pub struct ReadGuard;
    pub struct WriteGuard;
    pub struct RwLock;
    pub struct MappedReadGuard;
    pub struct CacheAligned;
    pub struct Lock;
    pub struct LockGuard;
    pub struct Mode;
    pub mod mode {}
}

pub mod stable_hasher {
    pub struct HashStable;
    pub struct StableHasher;
    pub struct ToStableHashKey;
    pub struct StableOrd;
    pub struct StableCompare;
    pub struct HashingControls;
}

pub mod graph {}
pub mod fingerprint {}
pub mod static_assert_size {}
pub mod tagged_ptr {}
pub mod intern {}
pub mod unhash {}
pub mod profiling {}
pub mod stack {}
pub mod undo_log {
    pub struct UndoLogs;
}

pub struct AtomicRef;
pub struct FatalErrorMarker;
