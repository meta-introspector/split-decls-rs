// Generated macro for ok (function)
macro_rules! Depcrate_readyok {
() => {
// Module: crate::ready
// Provides: {"ok"}
// Dependencies: {}
# [doc = " Create a future that is immediately ready with a success value."] # [allow (dead_code)] pub (crate) fn ok < T , E > (val : T) -> Ready < Result < T , E > > { Ready { val : Some (Ok (val)) } }
};
}
