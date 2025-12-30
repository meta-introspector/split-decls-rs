// Generated macro for Locked (enum)
macro_rules! Depcrate_rt_rwlockLocked {
() => {
// Module: crate::rt::rwlock
// Provides: {"Locked"}
// Dependencies: {}
# [derive (Debug , PartialEq)] enum Locked { Read (HashSet < thread :: Id >) , Write (thread :: Id) , }
};
}
