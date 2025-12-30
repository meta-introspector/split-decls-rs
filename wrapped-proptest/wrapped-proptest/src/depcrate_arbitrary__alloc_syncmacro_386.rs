// Generated macro for macro_386 (macro)
macro_rules! Depcrate_arbitrary__alloc_syncmacro_386 {
() => {
// Module: crate::arbitrary::_alloc::sync
// Provides: {"macro_386"}
// Dependencies: {}
arbitrary ! (Ordering , TupleUnion < (WA < Just < Self >>, WA < Just < Self >>, WA < Just < Self >>, WA < Just < Self >>, WA < Just < Self >>) >; prop_oneof ! [Just (Ordering :: Relaxed) , Just (Ordering :: Release) , Just (Ordering :: Acquire) , Just (Ordering :: AcqRel) , Just (Ordering :: SeqCst)]) ;
};
}
