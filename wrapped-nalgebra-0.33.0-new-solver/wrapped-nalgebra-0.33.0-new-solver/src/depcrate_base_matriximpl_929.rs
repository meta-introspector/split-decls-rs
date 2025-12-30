// Generated macro for impl_929 (impl)
macro_rules! Depcrate_base_matriximpl_929 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_929"}
// Dependencies: {}
impl < T , R : Dim , C : Dim , S > UlpsEq for Matrix < T , R , C , S > where T : Scalar + UlpsEq , S : RawStorage < T , R , C > , T :: Epsilon : Clone , { # [inline] fn default_max_ulps () -> u32 { T :: default_max_ulps () } # [inline] fn ulps_eq (& self , other : & Self , epsilon : Self :: Epsilon , max_ulps : u32) -> bool { assert ! (self . shape () == other . shape ()) ; self . iter () . zip (other . iter ()) . all (| (a , b) | a . ulps_eq (b , epsilon . clone () , max_ulps)) } }
};
}
