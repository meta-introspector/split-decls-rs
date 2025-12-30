// Generated macro for MRInner (enum)
macro_rules! Depcrate_nonblockMRInner {
() => {
// Module: crate::nonblock
// Provides: {"MRInner"}
// Dependencies: {}
enum MRInner { Ready (Result < Message , Error >) , Pending (task :: Waker) , Neither , }
};
}
