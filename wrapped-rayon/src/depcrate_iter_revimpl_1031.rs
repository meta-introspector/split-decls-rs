// Generated macro for impl_1031 (impl)
macro_rules! Depcrate_iter_revimpl_1031 {
() => {
// Module: crate::iter::rev
// Provides: {"impl_1031"}
// Dependencies: {}
impl < P > Producer for RevProducer < P > where P : Producer , { type Item = P :: Item ; type IntoIter = iter :: Rev < P :: IntoIter > ; fn into_iter (self) -> Self :: IntoIter { self . base . into_iter () . rev () } fn min_len (& self) -> usize { self . base . min_len () } fn max_len (& self) -> usize { self . base . max_len () } fn split_at (self , index : usize) -> (Self , Self) { let (left , right) = self . base . split_at (self . len - index) ; (RevProducer { base : right , len : index , } , RevProducer { base : left , len : self . len - index , } ,) } }
};
}
