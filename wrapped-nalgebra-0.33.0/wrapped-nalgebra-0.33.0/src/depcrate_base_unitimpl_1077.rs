// Generated macro for impl_1077 (impl)
macro_rules! Depcrate_base_unitimpl_1077 {
() => {
// Module: crate::base::unit
// Provides: {"impl_1077"}
// Dependencies: {}
impl < T , R , C , S > PartialEq for Unit < Matrix < T , R , C , S > > where T : Scalar + PartialEq , R : Dim , C : Dim , S : RawStorage < T , R , C > , { # [inline] fn eq (& self , rhs : & Self) -> bool { self . value . eq (& rhs . value) } }
};
}
