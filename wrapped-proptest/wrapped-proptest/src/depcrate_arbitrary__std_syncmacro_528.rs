// Generated macro for macro_528 (macro)
macro_rules! Depcrate_arbitrary__std_syncmacro_528 {
() => {
// Module: crate::arbitrary::_std::sync
// Provides: {"macro_528"}
// Dependencies: {}
arbitrary ! ([P : Clone + Default , T : Arbitrary < Parameters = P >] TrySendError < T >, TupleUnion < (WA < SMapped < T , Self >>, WA < SMapped < T , Self >>) >, P ; args => prop_oneof ! [static_map (any_with ::< T > (args . clone ()) , TrySendError :: Disconnected) , static_map (any_with ::< T > (args) , TrySendError :: Full) ,]) ;
};
}
