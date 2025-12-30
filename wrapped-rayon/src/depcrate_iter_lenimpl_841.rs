// Generated macro for impl_841 (impl)
macro_rules! Depcrate_iter_lenimpl_841 {
() => {
// Module: crate::iter::len
// Provides: {"impl_841"}
// Dependencies: {}
impl < P > Producer for MinLenProducer < P > where P : Producer , { type Item = P :: Item ; type IntoIter = P :: IntoIter ; fn into_iter (self) -> Self :: IntoIter { self . base . into_iter () } fn min_len (& self) -> usize { Ord :: max (self . min , self . base . min_len ()) } fn max_len (& self) -> usize { self . base . max_len () } fn split_at (self , index : usize) -> (Self , Self) { let (left , right) = self . base . split_at (index) ; (MinLenProducer { base : left , min : self . min , } , MinLenProducer { base : right , min : self . min , } ,) } fn fold_with < F > (self , folder : F) -> F where F : Folder < Self :: Item > , { self . base . fold_with (folder) } }
};
}
