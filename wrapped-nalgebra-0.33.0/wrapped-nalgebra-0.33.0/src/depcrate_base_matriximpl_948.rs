// Generated macro for impl_948 (impl)
macro_rules! Depcrate_base_matriximpl_948 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_948"}
// Dependencies: {}
impl < T , R : Dim , C : Dim , S > UlpsEq for Unit < Matrix < T , R , C , S > > where T : Scalar + UlpsEq , S : RawStorage < T , R , C > , T :: Epsilon : Clone , { # [inline] fn default_max_ulps () -> u32 { T :: default_max_ulps () } # [inline] fn ulps_eq (& self , other : & Self , epsilon : Self :: Epsilon , max_ulps : u32) -> bool { self . as_ref () . ulps_eq (other . as_ref () , epsilon , max_ulps) } }
};
}
