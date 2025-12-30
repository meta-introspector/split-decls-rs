// Generated macro for atomic (macro)
macro_rules! Depcrate_arbitrary__alloc_syncatomic {
() => {
// Module: crate::arbitrary::_alloc::sync
// Provides: {"atomic"}
// Dependencies: {}
macro_rules ! atomic { ($ ($ type : ident , $ base : ty) ;+) => { $ (arbitrary ! ($ type , SMapped <$ base , Self >; static_map (any ::<$ base > () , $ type :: new)) ;) + } ; }
};
}
