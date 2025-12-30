// Generated macro for test (module)
macro_rules! Depcrate_itertest {
() => {
// Module: crate::iter
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; fn send < I : Send > (_iter : I) { } # [test] fn test_send_iter () { send (GenericArray :: from ([1 , 2 , 3 , 4]) . into_iter ()) ; } }
};
}
