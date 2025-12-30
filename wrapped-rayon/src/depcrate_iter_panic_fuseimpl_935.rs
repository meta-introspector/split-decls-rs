// Generated macro for impl_935 (impl)
macro_rules! Depcrate_iter_panic_fuseimpl_935 {
() => {
// Module: crate::iter::panic_fuse
// Provides: {"impl_935"}
// Dependencies: {}
impl < 'a , P > Producer for PanicFuseProducer < 'a , P > where P : Producer , { type Item = P :: Item ; type IntoIter = PanicFuseIter < 'a , P :: IntoIter > ; fn into_iter (self) -> Self :: IntoIter { PanicFuseIter { base : self . base . into_iter () , fuse : self . fuse , } } fn min_len (& self) -> usize { self . base . min_len () } fn max_len (& self) -> usize { self . base . max_len () } fn split_at (self , index : usize) -> (Self , Self) { let (left , right) = self . base . split_at (index) ; (PanicFuseProducer { base : left , fuse : self . fuse . clone () , } , PanicFuseProducer { base : right , fuse : self . fuse , } ,) } fn fold_with < G > (self , folder : G) -> G where G : Folder < Self :: Item > , { let folder1 = PanicFuseFolder { base : folder , fuse : self . fuse , } ; self . base . fold_with (folder1) . base } }
};
}
