// Generated macro for macro_530 (macro)
macro_rules! Depcrate_arbitrary__std_syncmacro_530 {
() => {
// Module: crate::arbitrary::_std::sync
// Provides: {"macro_530"}
// Dependencies: {}
arbitrary ! ([A : fmt :: Debug] (Sender < A >, IntoIter < A >) , LazyJustFn < Self >; LazyJust :: new (|| { let (rx , tx) = channel () ; (rx , tx . into_iter ()) })) ;
};
}
