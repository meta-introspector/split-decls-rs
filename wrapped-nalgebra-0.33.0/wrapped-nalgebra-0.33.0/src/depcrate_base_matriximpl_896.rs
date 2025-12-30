// Generated macro for impl_896 (impl)
macro_rules! Depcrate_base_matriximpl_896 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_896"}
// Dependencies: {}
impl < T , R : Dim , C : Dim , S : fmt :: Debug > fmt :: Debug for Matrix < T , R , C , S > { fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { self . data . fmt (formatter) } }
};
}
