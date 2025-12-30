// Generated macro for ThreadData (struct)
macro_rules! Depcrate_word_lockThreadData {
() => {
// Module: crate::word_lock
// Provides: {"ThreadData"}
// Dependencies: {}
struct ThreadData { parker : ThreadParker , queue_tail : Cell < * const ThreadData > , prev : Cell < * const ThreadData > , next : Cell < * const ThreadData > , }
};
}
