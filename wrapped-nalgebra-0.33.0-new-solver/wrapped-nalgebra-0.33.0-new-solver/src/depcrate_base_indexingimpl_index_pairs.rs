// Generated macro for impl_index_pairs (macro)
macro_rules! Depcrate_base_indexingimpl_index_pairs {
() => {
// Module: crate::base::indexing
// Provides: {"impl_index_pairs"}
// Dependencies: {}
macro_rules ! impl_index_pairs { (index $ R : ident with { } index $ C : ident with { $ ($ r : tt ,) * }) => { } ; (index $ R : ident with { $ lh : tt , $ ($ lt : tt ,) * } index $ C : ident with { $ ($ r : tt ,) * }) => { $ (impl_index_pair ! { $ R , $ C , $ lh , $ r }) * impl_index_pairs ! { index $ R with { $ ($ lt ,) * } index $ C with { $ ($ r ,) * } } } }
};
}
