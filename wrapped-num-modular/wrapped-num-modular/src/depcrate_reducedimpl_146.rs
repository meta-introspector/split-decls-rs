// Generated macro for impl_146 (impl)
macro_rules! Depcrate_reducedimpl_146 {
() => {
// Module: crate::reduced
// Provides: {"impl_146"}
// Dependencies: {}
impl < T : PartialEq + Clone , R : Reducer < T > + Clone > ModularInteger for ReducedInt < T , R > { type Base = T ; # [inline] fn modulus (& self) -> T { self . r . modulus () } # [inline (always)] fn residue (& self) -> T { debug_assert ! (self . r . check (& self . a)) ; self . r . residue (self . a . clone ()) } # [inline (always)] fn is_zero (& self) -> bool { self . r . is_zero (& self . a) } # [inline] fn convert (& self , n : T) -> Self { Self { a : self . r . transform (n) , r : self . r . clone () , } } # [inline] fn double (self) -> Self { let Self { a , r } = self ; let a = r . dbl (a) ; Self { a , r } } # [inline] fn square (self) -> Self { let Self { a , r } = self ; let a = r . sqr (a) ; Self { a , r } } }
};
}
