// Generated macro for Kind (enum)
macro_rules! Depcrate_errorKind {
() => {
// Module: crate::error
// Provides: {"Kind"}
// Dependencies: {}
# [derive (Debug)] pub (crate) enum Kind { Builder , Request , Redirect , # [cfg (not (target_arch = "wasm32"))] Status (StatusCode , Option < hyper :: ext :: ReasonPhrase >) , # [cfg (target_arch = "wasm32")] Status (StatusCode) , Body , Decode , Upgrade , }
};
}
