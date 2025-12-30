// Generated macro for Map (struct)
macro_rules! Depcrate_rowMap {
() => {
// Module: crate::row
// Provides: {"Map"}
// Dependencies: {}
# [doc = " `F` is used to transform the _streaming_ iterator into a _fallible_"] # [doc = " iterator."] # [must_use = "iterators are lazy and do nothing unless consumed"] pub struct Map < 'stmt , F > { rows : Rows < 'stmt > , f : F , }
};
}
