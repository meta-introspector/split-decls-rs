// Generated macro for impl_782 (impl)
macro_rules! Depcrate_iter_inspectimpl_782 {
() => {
// Module: crate::iter::inspect
// Provides: {"impl_782"}
// Dependencies: {}
impl < 'f , P , F > Producer for InspectProducer < 'f , P , F > where P : Producer , F : Fn (& P :: Item) + Sync , { type Item = P :: Item ; type IntoIter = iter :: Inspect < P :: IntoIter , & 'f F > ; fn into_iter (self) -> Self :: IntoIter { self . base . into_iter () . inspect (self . inspect_op) } fn min_len (& self) -> usize { self . base . min_len () } fn max_len (& self) -> usize { self . base . max_len () } fn split_at (self , index : usize) -> (Self , Self) { let (left , right) = self . base . split_at (index) ; (InspectProducer { base : left , inspect_op : self . inspect_op , } , InspectProducer { base : right , inspect_op : self . inspect_op , } ,) } fn fold_with < G > (self , folder : G) -> G where G : Folder < Self :: Item > , { let folder1 = InspectFolder { base : folder , inspect_op : self . inspect_op , } ; self . base . fold_with (folder1) . base } }
};
}
