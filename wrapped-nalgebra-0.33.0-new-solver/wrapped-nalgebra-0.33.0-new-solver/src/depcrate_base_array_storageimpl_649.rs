// Generated macro for impl_649 (impl)
macro_rules! Depcrate_base_array_storageimpl_649 {
() => {
// Module: crate::base::array_storage
// Provides: {"impl_649"}
// Dependencies: {}
impl < T , const R1 : usize , const C1 : usize , const R2 : usize , const C2 : usize > ReshapableStorage < T , Const < R1 > , Const < C1 > , Const < R2 > , Const < C2 > > for ArrayStorage < T , R1 , C1 > where T : Scalar , Const < R1 > : ToTypenum , Const < C1 > : ToTypenum , Const < R2 > : ToTypenum , Const < C2 > : ToTypenum , < Const < R1 > as ToTypenum > :: Typenum : Mul < < Const < C1 > as ToTypenum > :: Typenum > , < Const < R2 > as ToTypenum > :: Typenum : Mul < < Const < C2 > as ToTypenum > :: Typenum , Output = typenum :: Prod < < Const < R1 > as ToTypenum > :: Typenum , < Const < C1 > as ToTypenum > :: Typenum , > , > , { type Output = ArrayStorage < T , R2 , C2 > ; fn reshape_generic (self , _ : Const < R2 > , _ : Const < C2 >) -> Self :: Output { unsafe { let data : [[T ; R2] ; C2] = mem :: transmute_copy (& self . 0) ; mem :: forget (self . 0) ; ArrayStorage (data) } } }
};
}
