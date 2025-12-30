// Generated macro for lazy_just (macro)
macro_rules! Depcrate_arbitrary_macroslazy_just {
() => {
// Module: crate::arbitrary::macros
// Provides: {"lazy_just"}
// Dependencies: {}
macro_rules ! lazy_just { ($ ($ self : ty , $ fun : expr) ;+) => { $ (arbitrary ! ($ self , $ crate :: strategy :: LazyJust < Self , fn () -> Self >; $ crate :: strategy :: LazyJust :: new ($ fun)) ;) + } ; }
};
}
