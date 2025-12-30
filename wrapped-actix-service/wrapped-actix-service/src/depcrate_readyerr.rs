// Generated macro for err (function)
macro_rules! Depcrate_readyerr {
() => {
// Module: crate::ready
// Provides: {"err"}
// Dependencies: {}
# [doc = " Create a future that is immediately ready with an error value."] # [allow (dead_code)] pub (crate) fn err < T , E > (err : E) -> Ready < Result < T , E > > { Ready { val : Some (Err (err)) , } }
};
}
