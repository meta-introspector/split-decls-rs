// Generated macro for ColumnFamilyRef (type)
macro_rules! Depcrate_column_familyColumnFamilyRef {
() => {
// Module: crate::column_family
// Provides: {"ColumnFamilyRef"}
// Dependencies: {}
# [cfg (feature = "multi-threaded-cf")] pub type ColumnFamilyRef < 'a > = Arc < BoundColumnFamily < 'a > > ;
};
}
