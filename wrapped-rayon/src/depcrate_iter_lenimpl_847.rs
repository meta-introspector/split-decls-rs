// Generated macro for impl_847 (impl)
macro_rules! Depcrate_iter_lenimpl_847 {
() => {
// Module: crate::iter::len
// Provides: {"impl_847"}
// Dependencies: {}
impl < P > Producer for MaxLenProducer < P > where P : Producer , { type Item = P :: Item ; type IntoIter = P :: IntoIter ; fn into_iter (self) -> Self :: IntoIter { self . base . into_iter () } fn min_len (& self) -> usize { self . base . min_len () } fn max_len (& self) -> usize { Ord :: min (self . max , self . base . max_len ()) } fn split_at (self , index : usize) -> (Self , Self) { let (left , right) = self . base . split_at (index) ; (MaxLenProducer { base : left , max : self . max , } , MaxLenProducer { base : right , max : self . max , } ,) } fn fold_with < F > (self , folder : F) -> F where F : Folder < Self :: Item > , { self . base . fold_with (folder) } }
};
}
