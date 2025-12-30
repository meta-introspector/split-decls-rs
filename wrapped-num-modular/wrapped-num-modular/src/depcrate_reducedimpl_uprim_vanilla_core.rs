// Generated macro for impl_uprim_vanilla_core (macro)
macro_rules! Depcrate_reducedimpl_uprim_vanilla_core {
() => {
// Module: crate::reduced
// Provides: {"impl_uprim_vanilla_core"}
// Dependencies: {}
macro_rules ! impl_uprim_vanilla_core { ($ single : ty) => { # [inline (always)] fn new (m : &$ single) -> Self { assert ! (m > & 0) ; Self (* m) } # [inline (always)] fn transform (& self , target : $ single) -> $ single { target % self . 0 } # [inline (always)] fn check (& self , target : &$ single) -> bool { * target < self . 0 } # [inline (always)] fn residue (& self , target : $ single) -> $ single { target } # [inline (always)] fn modulus (& self) -> $ single { self . 0 } # [inline (always)] fn is_zero (& self , target : &$ single) -> bool { * target == 0 } # [inline (always)] fn add (& self , lhs : &$ single , rhs : &$ single) -> $ single { Vanilla ::<$ single >:: add (& self . 0 , * lhs , * rhs) } # [inline (always)] fn dbl (& self , target : $ single) -> $ single { Vanilla ::<$ single >:: dbl (& self . 0 , target) } # [inline (always)] fn sub (& self , lhs : &$ single , rhs : &$ single) -> $ single { Vanilla ::<$ single >:: sub (& self . 0 , * lhs , * rhs) } # [inline (always)] fn neg (& self , target : $ single) -> $ single { Vanilla ::<$ single >:: neg (& self . 0 , target) } # [inline (always)] fn inv (& self , target : $ single) -> Option <$ single > { target . invm (& self . 0) } impl_reduced_binary_pow ! ($ single) ; } ; }
};
}
