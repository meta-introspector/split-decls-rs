// Generated macro for impl_593 (impl)
macro_rules! Depcrate_iter_filter_mapimpl_593 {
() => {
// Module: crate::iter::filter_map
// Provides: {"impl_593"}
// Dependencies: {}
impl < 'p , T , U , C , P > Consumer < T > for FilterMapConsumer < 'p , C , P > where C : Consumer < U > , P : Fn (T) -> Option < U > + Sync + 'p , { type Folder = FilterMapFolder < 'p , C :: Folder , P > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , Self :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (FilterMapConsumer :: new (left , self . filter_op) , FilterMapConsumer :: new (right , self . filter_op) , reducer ,) } fn into_folder (self) -> Self :: Folder { let base = self . base . into_folder () ; FilterMapFolder { base , filter_op : self . filter_op , } } fn full (& self) -> bool { self . base . full () } }
};
}
