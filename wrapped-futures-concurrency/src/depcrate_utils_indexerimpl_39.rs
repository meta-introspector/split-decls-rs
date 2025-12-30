// Generated macro for impl_39 (impl)
macro_rules! Depcrate_utils_indexerimpl_39 {
() => {
// Module: crate::utils::indexer
// Provides: {"impl_39"}
// Dependencies: {}
impl Indexer { pub (crate) fn new (max : usize) -> Self { Self { offset : 0 , max } } # [doc = " Generate a range between `0..max`, incrementing the starting point"] # [doc = " for the next iteration."] pub (crate) fn iter (& mut self) -> IndexIter { let offset = self . offset ; self . offset = (self . offset + 1) . wrapping_rem (self . max) ; IndexIter { iter : (0 .. self . max) , offset , } } }
};
}
