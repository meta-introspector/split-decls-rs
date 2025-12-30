// Generated macro for AtomicRwOrd (enum)
macro_rules! Depcrate_concurrency_data_raceAtomicRwOrd {
() => {
// Module: crate::concurrency::data_race
// Provides: {"AtomicRwOrd"}
// Dependencies: {}
# [doc = " Valid atomic read-write orderings, alias of atomic::Ordering (not non-exhaustive)."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] pub enum AtomicRwOrd { Relaxed , Acquire , Release , AcqRel , SeqCst , }
};
}
