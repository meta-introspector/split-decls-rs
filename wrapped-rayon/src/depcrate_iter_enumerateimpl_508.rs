// Generated macro for impl_508 (impl)
macro_rules! Depcrate_iter_enumerateimpl_508 {
() => {
// Module: crate::iter::enumerate
// Provides: {"impl_508"}
// Dependencies: {}
impl < P > Producer for EnumerateProducer < P > where P : Producer , { type Item = (usize , P :: Item) ; type IntoIter = iter :: Zip < Range < usize > , P :: IntoIter > ; fn into_iter (self) -> Self :: IntoIter { let base = self . base . into_iter () ; let end = self . offset + base . len () ; (self . offset .. end) . zip (base) } fn min_len (& self) -> usize { self . base . min_len () } fn max_len (& self) -> usize { self . base . max_len () } fn split_at (self , index : usize) -> (Self , Self) { let (left , right) = self . base . split_at (index) ; (EnumerateProducer { base : left , offset : self . offset , } , EnumerateProducer { base : right , offset : self . offset + index , } ,) } }
};
}
