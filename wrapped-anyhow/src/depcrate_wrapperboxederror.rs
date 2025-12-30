// Generated macro for BoxedError (struct)
macro_rules! Depcrate_wrapperBoxedError {
() => {
// Module: crate::wrapper
// Provides: {"BoxedError"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (anyhow_no_core_error)))] # [repr (transparent)] pub struct BoxedError (pub Box < dyn StdError + Send + Sync >) ;
};
}
