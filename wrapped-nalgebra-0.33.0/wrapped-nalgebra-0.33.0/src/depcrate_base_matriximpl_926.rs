// Generated macro for impl_926 (impl)
macro_rules! Depcrate_base_matriximpl_926 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_926"}
// Dependencies: {}
impl < T : Scalar , D : DimAdd < U1 > , S : RawStorage < T , D > > Vector < T , D , S > { # [doc = " Constructs a new vector of higher dimension by appending `element` to the end of `self`."] # [inline] # [must_use] pub fn push (& self , element : T) -> OVector < T , DimSum < D , U1 > > where DefaultAllocator : Allocator < DimSum < D , U1 > > , { let len = self . len () ; let hnrows = DimSum :: < D , U1 > :: from_usize (len + 1) ; let mut res = Matrix :: uninit (hnrows , Const :: < 1 >) ; res . generic_view_mut ((0 , 0) , self . shape_generic ()) . zip_apply (self , | out , e | * out = MaybeUninit :: new (e)) ; res [(len , 0)] = MaybeUninit :: new (element) ; unsafe { res . assume_init () } } }
};
}
