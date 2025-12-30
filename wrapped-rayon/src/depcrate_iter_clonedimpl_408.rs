// Generated macro for impl_408 (impl)
macro_rules! Depcrate_iter_clonedimpl_408 {
() => {
// Module: crate::iter::cloned
// Provides: {"impl_408"}
// Dependencies: {}
impl < 'a , T , P > Producer for ClonedProducer < P > where P : Producer < Item = & 'a T > , T : 'a + Clone , { type Item = T ; type IntoIter = iter :: Cloned < P :: IntoIter > ; fn into_iter (self) -> Self :: IntoIter { self . base . into_iter () . cloned () } fn min_len (& self) -> usize { self . base . min_len () } fn max_len (& self) -> usize { self . base . max_len () } fn split_at (self , index : usize) -> (Self , Self) { let (left , right) = self . base . split_at (index) ; (ClonedProducer { base : left } , ClonedProducer { base : right } ,) } fn fold_with < F > (self , folder : F) -> F where F : Folder < Self :: Item > , { self . base . fold_with (ClonedFolder { base : folder }) . base } }
};
}
