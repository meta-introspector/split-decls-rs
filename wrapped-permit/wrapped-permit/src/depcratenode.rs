// Generated macro for Node (struct)
macro_rules! DepcrateNode {
() => {
// Module: crate
// Provides: {"Node"}
// Dependencies: {}
struct Node { superior : Weak < Node > , atomic_revoked : AtomicBool , inner : Mutex < Inner > , condvar : Condvar , }
};
}
