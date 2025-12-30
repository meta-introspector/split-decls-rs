// Generated macro for Node (struct)
macro_rules! Depcrate_mpsc_queueNode {
() => {
// Module: crate::mpsc::queue
// Provides: {"Node"}
// Dependencies: {}
struct Node < T > { next : AtomicPtr < Self > , value : Option < T > , }
};
}
