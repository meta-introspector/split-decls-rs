// Generated macro for Store (struct)
macro_rules! Depcrate_rt_atomicStore {
() => {
// Module: crate::rt::atomic
// Provides: {"Store"}
// Dependencies: {}
# [derive (Debug)] struct Store { # [doc = " The stored value. All atomic types can be converted to `u64`."] value : u64 , # [doc = " The causality of the thread when it stores the value."] happens_before : VersionVec , # [doc = " Tracks the modification order. Order is tracked as a partially-ordered"] # [doc = " set."] modification_order : VersionVec , # [doc = " Manages causality transfers between threads"] sync : Synchronize , # [doc = " Tracks when each thread first saw value"] first_seen : FirstSeen , # [doc = " True when the store was done with `SeqCst` ordering"] seq_cst : bool , }
};
}
