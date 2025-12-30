// Generated macro for impl_from_into_mint_1D (macro)
macro_rules! Depcrate_third_party_mint_mint_matriximpl_from_into_mint_1D {
() => {
// Module: crate::third_party::mint::mint_matrix
// Provides: {"impl_from_into_mint_1D"}
// Dependencies: {}
macro_rules ! impl_from_into_mint_1D (($ ($ NRows : ident => $ VT : ident [$ SZ : expr]) ;* $ (;) *) => { $ (impl < T > From < mint ::$ VT < T >> for OMatrix < T , $ NRows , U1 > where T : Scalar , DefaultAllocator : Allocator <$ NRows , U1 > { # [inline] fn from (v : mint ::$ VT < T >) -> Self { unsafe { let mut res = Matrix :: uninit (<$ NRows >:: name () , Const ::< 1 >) ; ptr :: copy_nonoverlapping (& v . x , res . data . ptr_mut () as * mut T , $ SZ) ; mem :: forget (v) ; res . assume_init () } } } impl < T , S > Into < mint ::$ VT < T >> for Matrix < T , $ NRows , U1 , S > where T : Scalar , S : RawStorage < T , $ NRows , U1 > + IsContiguous { # [inline] fn into (self) -> mint ::$ VT < T > { unsafe { let mut res : MaybeUninit < mint ::$ VT < T >> = MaybeUninit :: uninit () ; ptr :: copy_nonoverlapping (self . data . ptr () , res . as_mut_ptr () as * mut T , $ SZ) ; mem :: forget (self) ; res . assume_init () } } } impl < T , S > AsRef < mint ::$ VT < T >> for Matrix < T , $ NRows , U1 , S > where T : Scalar , S : RawStorage < T , $ NRows , U1 > + IsContiguous { # [inline] fn as_ref (& self) -> & mint ::$ VT < T > { unsafe { mem :: transmute (self . data . ptr ()) } } } impl < T , S > AsMut < mint ::$ VT < T >> for Matrix < T , $ NRows , U1 , S > where T : Scalar , S : RawStorageMut < T , $ NRows , U1 > + IsContiguous { # [inline] fn as_mut (& mut self) -> & mut mint ::$ VT < T > { unsafe { mem :: transmute (self . data . ptr_mut ()) } } }) * }) ;
};
}
