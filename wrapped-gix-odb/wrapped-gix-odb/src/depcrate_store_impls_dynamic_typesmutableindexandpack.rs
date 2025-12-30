// Generated macro for MutableIndexAndPack (struct)
macro_rules! Depcrate_store_impls_dynamic_typesMutableIndexAndPack {
() => {
// Module: crate::store_impls::dynamic::types
// Provides: {"MutableIndexAndPack"}
// Dependencies: {}
# [derive (Default)] pub (crate) struct MutableIndexAndPack { pub (crate) files : ArcSwap < Option < IndexAndPacks > > , pub (crate) write : parking_lot :: Mutex < () > , # [doc = " The generation required at least to read this slot. If these mismatch, the caller is likely referring to a now changed slot"] # [doc = " that has different content under the same id."] # [doc = " Must only be changed when the write lock is held."] pub (crate) generation : AtomicGeneration , }
};
}
