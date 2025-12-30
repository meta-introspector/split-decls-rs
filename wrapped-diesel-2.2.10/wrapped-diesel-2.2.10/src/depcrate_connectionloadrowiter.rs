// Generated macro for LoadRowIter (type)
macro_rules! Depcrate_connectionLoadRowIter {
() => {
// Module: crate::connection
// Provides: {"LoadRowIter"}
// Dependencies: {}
# [doc (hidden)] # [cfg (all (feature = "with-deprecated" , not (feature = "without-deprecated")))] # [deprecated (note = "Directly use `LoadConnection::Cursor` instead")] pub type LoadRowIter < 'conn , 'query , C , DB , B = DefaultLoadingMode > = < C as self :: private :: ConnectionHelperType < DB , B > > :: Cursor < 'conn , 'query > ;
};
}
