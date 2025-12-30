// Generated macro for impl_1237 (impl)
macro_rules! Depcrate_iter_updateimpl_1237 {
() => {
// Module: crate::iter::update
// Provides: {"impl_1237"}
// Dependencies: {}
impl < 'f , P , F > Producer for UpdateProducer < 'f , P , F > where P : Producer , F : Fn (& mut P :: Item) + Send + Sync , { type Item = P :: Item ; type IntoIter = UpdateSeq < P :: IntoIter , & 'f F > ; fn into_iter (self) -> Self :: IntoIter { UpdateSeq { base : self . base . into_iter () , update_op : self . update_op , } } fn min_len (& self) -> usize { self . base . min_len () } fn max_len (& self) -> usize { self . base . max_len () } fn split_at (self , index : usize) -> (Self , Self) { let (left , right) = self . base . split_at (index) ; (UpdateProducer { base : left , update_op : self . update_op , } , UpdateProducer { base : right , update_op : self . update_op , } ,) } fn fold_with < G > (self , folder : G) -> G where G : Folder < Self :: Item > , { let folder1 = UpdateFolder { base : folder , update_op : self . update_op , } ; self . base . fold_with (folder1) . base } }
};
}
