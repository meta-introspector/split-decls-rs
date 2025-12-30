// Generated macro for impl_129 (impl)
macro_rules! Depcrate_reducedimpl_129 {
() => {
// Module: crate::reduced
// Provides: {"impl_129"}
// Dependencies: {}
impl < T , R : Reducer < T > > ReducedInt < T , R > { # [doc = " Convert n into the modulo ring ℤ/mℤ (i.e. `n % m`)"] # [inline] pub fn new (n : T , m : & T) -> Self { let r = R :: new (m) ; let a = r . transform (n) ; Self { a , r } } # [inline (always)] fn check_modulus_eq (& self , rhs : & Self) where T : PartialEq , { if cfg ! (debug_assertions) && self . r . modulus () != rhs . r . modulus () { panic ! ("The modulus of two operators should be the same!") ; } } # [inline (always)] pub fn repr (& self) -> & T { & self . a } # [inline (always)] pub fn inv (self) -> Option < Self > { Some (Self { a : self . r . inv (self . a) ? , r : self . r , }) } # [inline (always)] pub fn pow (self , exp : & T) -> Self { Self { a : self . r . pow (self . a , exp) , r : self . r , } } }
};
}
