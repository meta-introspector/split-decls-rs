// Generated macro for impl_896 (impl)
macro_rules! Depcrate_iter_map_withimpl_896 {
() => {
// Module: crate::iter::map_with
// Provides: {"impl_896"}
// Dependencies: {}
impl < 'f , T , INIT , U , R , C , F > Consumer < T > for MapInitConsumer < 'f , C , INIT , F > where C : Consumer < R > , INIT : Fn () -> U + Sync , F : Fn (& mut U , T) -> R + Sync , R : Send , { type Folder = MapWithFolder < 'f , C :: Folder , U , F > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , Self :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (MapInitConsumer :: new (left , self . init , self . map_op) , MapInitConsumer :: new (right , self . init , self . map_op) , reducer ,) } fn into_folder (self) -> Self :: Folder { MapWithFolder { base : self . base . into_folder () , item : (self . init) () , map_op : self . map_op , } } fn full (& self) -> bool { self . base . full () } }
};
}
