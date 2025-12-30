// Generated macro for OwnedOrStaticAtomicBool (enum)
macro_rules! Depcrate_utilOwnedOrStaticAtomicBool {
() => {
// Module: crate::util
// Provides: {"OwnedOrStaticAtomicBool"}
// Dependencies: {}
# [derive (Clone)] pub enum OwnedOrStaticAtomicBool { Owned { flag : Arc < AtomicBool > , # [cfg_attr (not (feature = "parallel") , allow (dead_code))] private : bool , } , Shared (& 'static AtomicBool) , }
};
}
