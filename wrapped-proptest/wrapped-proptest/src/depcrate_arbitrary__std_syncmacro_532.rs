// Generated macro for macro_532 (macro)
macro_rules! Depcrate_arbitrary__std_syncmacro_532 {
() => {
// Module: crate::arbitrary::_std::sync
// Provides: {"macro_532"}
// Dependencies: {}
arbitrary ! ([A : fmt :: Debug] (SyncSender < A >, IntoIter < A >) , SMapped < u16 , Self >; static_map (any ::< u16 > () , | size | { let (rx , tx) = sync_channel (size as usize) ; (rx , tx . into_iter ()) })) ;
};
}
