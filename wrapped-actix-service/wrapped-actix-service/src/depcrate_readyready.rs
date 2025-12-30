// Generated macro for ready (function)
macro_rules! Depcrate_readyready {
() => {
// Module: crate::ready
// Provides: {"ready"}
// Dependencies: {}
# [doc = " Creates a future that is immediately ready with a value."] # [allow (dead_code)] pub (crate) fn ready < T > (val : T) -> Ready < T > { Ready { val : Some (val) } }
};
}
