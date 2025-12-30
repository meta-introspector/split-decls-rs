// Generated macro for Pool (struct)
macro_rules! Depcrate_poolPool {
() => {
// Module: crate::pool
// Provides: {"Pool"}
// Dependencies: {}
# [doc = " A thread safe pool utilizing alloc-only features."] # [doc = ""] # [doc = " Unlike the pool in regex-automata, this has no \"fast path.\" We could add"] # [doc = " it, but it's more code and requires reasoning about safety."] pub (crate) struct Pool < T , F > { # [doc = " A stack of T values to hand out. These are used when a Pool is"] # [doc = " accessed by a thread that didn't create it."] stack : Mutex < Vec < Box < T > > > , # [doc = " A function to create more T values when stack is empty and a caller"] # [doc = " has requested a T."] create : F , }
};
}
