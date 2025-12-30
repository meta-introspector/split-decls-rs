// Generated macro for macro_526 (macro)
macro_rules! Depcrate_arbitrary__std_syncmacro_526 {
() => {
// Module: crate::arbitrary::_std::sync
// Provides: {"macro_526"}
// Dependencies: {}
arbitrary ! (RecvTimeoutError , TupleUnion < (WA < Just < Self >>, WA < Just < Self >>) >; prop_oneof ! [Just (RecvTimeoutError :: Disconnected) , Just (RecvTimeoutError :: Timeout)]) ;
};
}
