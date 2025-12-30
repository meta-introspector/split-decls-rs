// Generated macro for impl_862 (impl)
macro_rules! Depcrate_iter_mapimpl_862 {
() => {
// Module: crate::iter::map
// Provides: {"impl_862"}
// Dependencies: {}
impl < 'f , T , R , C , F > Consumer < T > for MapConsumer < 'f , C , F > where C : Consumer < F :: Output > , F : Fn (T) -> R + Sync , R : Send , { type Folder = MapFolder < 'f , C :: Folder , F > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , Self :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (MapConsumer :: new (left , self . map_op) , MapConsumer :: new (right , self . map_op) , reducer ,) } fn into_folder (self) -> Self :: Folder { MapFolder { base : self . base . into_folder () , map_op : self . map_op , } } fn full (& self) -> bool { self . base . full () } }
};
}
