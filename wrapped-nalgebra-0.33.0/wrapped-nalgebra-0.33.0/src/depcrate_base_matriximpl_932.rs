// Generated macro for impl_932 (impl)
macro_rules! Depcrate_base_matriximpl_932 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_932"}
// Dependencies: {}
impl < T , R , R2 , C , C2 , S , S2 > PartialEq < Matrix < T , R2 , C2 , S2 > > for Matrix < T , R , C , S > where T : PartialEq , C : Dim , C2 : Dim , R : Dim , R2 : Dim , S : RawStorage < T , R , C > , S2 : RawStorage < T , R2 , C2 > , { # [inline] fn eq (& self , right : & Matrix < T , R2 , C2 , S2 >) -> bool { self . shape () == right . shape () && self . iter () . zip (right . iter ()) . all (| (l , r) | l == r) } }
};
}
