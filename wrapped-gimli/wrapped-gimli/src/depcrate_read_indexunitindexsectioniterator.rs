// Generated macro for UnitIndexSectionIterator (struct)
macro_rules! Depcrate_read_indexUnitIndexSectionIterator {
() => {
// Module: crate::read::index
// Provides: {"UnitIndexSectionIterator"}
// Dependencies: {}
# [doc = " An iterator over the section offsets and sizes for a row in a `UnitIndex`."] # [derive (Debug , Clone)] pub struct UnitIndexSectionIterator < 'index , R : Reader > { sections : slice :: Iter < 'index , IndexSectionId > , offsets : R , sizes : R , }
};
}
