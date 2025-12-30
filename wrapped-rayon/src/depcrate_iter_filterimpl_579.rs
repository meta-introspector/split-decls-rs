// Generated macro for impl_579 (impl)
macro_rules! Depcrate_iter_filterimpl_579 {
() => {
// Module: crate::iter::filter
// Provides: {"impl_579"}
// Dependencies: {}
impl < 'p , T , C , P : 'p > Consumer < T > for FilterConsumer < 'p , C , P > where C : Consumer < T > , P : Fn (& T) -> bool + Sync , { type Folder = FilterFolder < 'p , C :: Folder , P > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , C :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (FilterConsumer :: new (left , self . filter_op) , FilterConsumer :: new (right , self . filter_op) , reducer ,) } fn into_folder (self) -> Self :: Folder { FilterFolder { base : self . base . into_folder () , filter_op : self . filter_op , } } fn full (& self) -> bool { self . base . full () } }
};
}
