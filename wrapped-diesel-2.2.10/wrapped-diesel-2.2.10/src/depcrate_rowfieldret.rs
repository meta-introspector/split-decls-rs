// Generated macro for FieldRet (type)
macro_rules! Depcrate_rowFieldRet {
() => {
// Module: crate::row
// Provides: {"FieldRet"}
// Dependencies: {}
# [doc (hidden)] # [cfg (all (feature = "with-deprecated" , not (feature = "without-deprecated")))] # [deprecated (note = "Use `Row::Field` directly instead")] pub type FieldRet < 'a , R , DB > = < R as self :: private :: RowLifetimeHelper < DB > > :: Field < 'a > ;
};
}
