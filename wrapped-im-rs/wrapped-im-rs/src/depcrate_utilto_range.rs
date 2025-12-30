// Generated macro for to_range (function)
macro_rules! Depcrate_utilto_range {
() => {
// Module: crate::util
// Provides: {"to_range"}
// Dependencies: {}
pub (crate) fn to_range < R > (range : & R , right_unbounded : usize) -> Range < usize > where R : RangeBounds < usize > , { let start_index = match range . start_bound () { Bound :: Included (i) => * i , Bound :: Excluded (i) => * i + 1 , Bound :: Unbounded => 0 , } ; let end_index = match range . end_bound () { Bound :: Included (i) => * i + 1 , Bound :: Excluded (i) => * i , Bound :: Unbounded => right_unbounded , } ; start_index .. end_index }
};
}
