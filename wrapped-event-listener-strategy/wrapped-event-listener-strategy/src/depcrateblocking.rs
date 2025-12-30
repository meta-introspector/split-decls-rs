// Generated macro for Blocking (struct)
macro_rules! DepcrateBlocking {
() => {
// Module: crate
// Provides: {"Blocking"}
// Dependencies: {}
# [doc = " A strategy that blocks the current thread until the event is signalled."] # [derive (Debug , Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Default)] # [cfg (all (feature = "std" , not (target_family = "wasm")))] pub struct Blocking { _private : () , }
};
}
