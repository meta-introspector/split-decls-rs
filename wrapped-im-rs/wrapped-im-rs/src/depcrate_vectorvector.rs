// Generated macro for vector (macro)
macro_rules! Depcrate_vectorvector {
() => {
// Module: crate::vector
// Provides: {"vector"}
// Dependencies: {}
# [doc = " Construct a vector from a sequence of elements."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate im;"] # [doc = " # use im::vector::Vector;"] # [doc = " # fn main() {"] # [doc = " assert_eq!("] # [doc = "   vector![1, 2, 3],"] # [doc = "   Vector::from(vec![1, 2, 3])"] # [doc = " );"] # [doc = " # }"] # [doc = " ```"] # [macro_export] macro_rules ! vector { () => { $ crate :: vector :: Vector :: new () } ; ($ ($ x : expr) ,*) => { { let mut l = $ crate :: vector :: Vector :: new () ; $ (l . push_back ($ x) ;) * l } } ; ($ ($ x : expr ,) *) => { { let mut l = $ crate :: vector :: Vector :: new () ; $ (l . push_back ($ x) ;) * l } } ; }
};
}
