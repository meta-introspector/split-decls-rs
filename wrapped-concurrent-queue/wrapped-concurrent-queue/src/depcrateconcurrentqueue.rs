// Generated macro for ConcurrentQueue (struct)
macro_rules! DepcrateConcurrentQueue {
() => {
// Module: crate
// Provides: {"ConcurrentQueue"}
// Dependencies: {}
# [doc = " A concurrent queue."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use concurrent_queue::{ConcurrentQueue, PopError, PushError};"] # [doc = ""] # [doc = " let q = ConcurrentQueue::bounded(2);"] # [doc = ""] # [doc = " assert_eq!(q.push('a'), Ok(()));"] # [doc = " assert_eq!(q.push('b'), Ok(()));"] # [doc = " assert_eq!(q.push('c'), Err(PushError::Full('c')));"] # [doc = ""] # [doc = " assert_eq!(q.pop(), Ok('a'));"] # [doc = " assert_eq!(q.pop(), Ok('b'));"] # [doc = " assert_eq!(q.pop(), Err(PopError::Empty));"] # [doc = " ```"] pub struct ConcurrentQueue < T > (Inner < T >) ;
};
}
