// Generated macro for debug_executor (function)
macro_rules! Depcratedebug_executor {
() => {
// Module: crate
// Provides: {"debug_executor"}
// Dependencies: {}
# [doc = " Debug implementation for `Executor` and `LocalExecutor`."] fn debug_executor (executor : & Executor < '_ > , name : & str , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let ptr = executor . state . load (Ordering :: Acquire) ; if ptr . is_null () { struct Uninitialized ; impl fmt :: Debug for Uninitialized { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("<uninitialized>") } } return f . debug_tuple (name) . field (& Uninitialized) . finish () ; } let state = unsafe { & * ptr } ; debug_state (state , name , f) }
};
}
