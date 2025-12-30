// Generated macro for MappedRows (struct)
macro_rules! Depcrate_rowMappedRows {
() => {
// Module: crate::row
// Provides: {"MappedRows"}
// Dependencies: {}
# [doc = " An iterator over the mapped resulting rows of a query."] # [doc = ""] # [doc = " `F` is used to transform the _streaming_ iterator into a _standard_"] # [doc = " iterator."] # [must_use = "iterators are lazy and do nothing unless consumed"] pub struct MappedRows < 'stmt , F > { rows : Rows < 'stmt > , map : F , }
};
}
