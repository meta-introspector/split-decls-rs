// Generated macro for impl_71 (impl)
macro_rules! Depcrate_word_lockimpl_71 {
() => {
// Module: crate::word_lock
// Provides: {"impl_71"}
// Dependencies: {}
impl ThreadData { # [inline] fn new () -> ThreadData { assert ! (mem :: align_of ::< ThreadData > () > ! QUEUE_MASK) ; ThreadData { parker : ThreadParker :: new () , queue_tail : Cell :: new (ptr :: null ()) , prev : Cell :: new (ptr :: null ()) , next : Cell :: new (ptr :: null ()) , } } }
};
}
