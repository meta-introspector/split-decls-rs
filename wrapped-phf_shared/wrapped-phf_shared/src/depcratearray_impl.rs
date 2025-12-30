// Generated macro for array_impl (macro)
macro_rules! Depcratearray_impl {
() => {
// Module: crate
// Provides: {"array_impl"}
// Dependencies: {}
macro_rules ! array_impl (($ t : ty) => (impl < const N : usize > PhfHash for [$ t ; N] { # [inline] fn phf_hash < H : Hasher > (& self , state : & mut H) { for v in & self [..] { v . phf_hash (state) ; } } } impl < const N : usize > FmtConst for [$ t ; N] { fn fmt_const (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { fmt_array (self , f) } } impl < const N : usize > PhfBorrow < [$ t] > for [$ t ; N] { fn borrow (& self) -> & [$ t] { self } })) ;
};
}
