// Generated macro for into_par_vec (macro)
macro_rules! Depcrate_collectionsinto_par_vec {
() => {
// Module: crate::collections
// Provides: {"into_par_vec"}
// Dependencies: {}
# [doc = " Convert an iterable collection into a parallel iterator by first"] # [doc = " collecting into a temporary `Vec`, then iterating that."] macro_rules ! into_par_vec { ($ t : ty => $ iter : ident <$ ($ i : tt) ,*>, impl $ ($ args : tt) *) => { impl $ ($ args) * IntoParallelIterator for $ t { type Item = <$ t as IntoIterator >:: Item ; type Iter = $ iter <$ ($ i) ,*>; fn into_par_iter (self) -> Self :: Iter { use std :: iter :: FromIterator ; $ iter { inner : Vec :: from_iter (self) . into_par_iter () } } } } ; }
};
}
