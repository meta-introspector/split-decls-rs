// Generated macro for impl_3195 (impl)
macro_rules! Depcrate_third_party_alga_alga_matriximpl_3195 {
() => {
// Module: crate::third_party::alga::alga_matrix
// Provides: {"impl_3195"}
// Dependencies: {}
impl < T , R : Dim , C : Dim > Lattice for OMatrix < T , R , C > where T : Scalar + Lattice , DefaultAllocator : Allocator < R , C > , { # [inline] fn meet_join (& self , other : & Self) -> (Self , Self) { let shape = self . shape_generic () ; assert ! (shape == other . shape_generic () , "Matrix meet/join error: mismatched dimensions.") ; let mut mres = Matrix :: uninit (shape . 0 , shape . 1) ; let mut jres = Matrix :: uninit (shape . 0 , shape . 1) ; for i in 0 .. shape . 0 . value () * shape . 1 . value () { unsafe { let mj = self . data . get_unchecked_linear (i) . meet_join (other . data . get_unchecked_linear (i)) ; * mres . data . get_unchecked_linear_mut (i) = MaybeUninit :: new (mj . 0) ; * jres . data . get_unchecked_linear_mut (i) = MaybeUninit :: new (mj . 1) ; } } unsafe { (mres . assume_init () , jres . assume_init ()) } } }
};
}
