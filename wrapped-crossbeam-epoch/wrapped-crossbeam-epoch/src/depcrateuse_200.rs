// Generated macro for use_200 (pub_use)
macro_rules! Depcrateuse_200 {
() => {
// Module: crate
// Provides: {"use_200"}
// Dependencies: {}
# [cfg (all (feature = "alloc" , target_has_atomic = "ptr"))] pub use crate :: { atomic :: { Atomic , CompareExchangeError , Owned , Pointable , Pointer , Shared } , collector :: { Collector , LocalHandle } , guard :: { unprotected , Guard } , } ;
};
}
