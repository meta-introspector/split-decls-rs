// Generated macro for IndexConstraintAndUsageIter (struct)
macro_rules! Depcrate_vtabIndexConstraintAndUsageIter {
() => {
// Module: crate::vtab
// Provides: {"IndexConstraintAndUsageIter"}
// Dependencies: {}
# [doc = " Iterate on index constraint and its associated usage."] pub struct IndexConstraintAndUsageIter < 'a > { iter : std :: iter :: Zip < slice :: Iter < 'a , ffi :: sqlite3_index_constraint > , slice :: IterMut < 'a , ffi :: sqlite3_index_constraint_usage > , > , }
};
}
