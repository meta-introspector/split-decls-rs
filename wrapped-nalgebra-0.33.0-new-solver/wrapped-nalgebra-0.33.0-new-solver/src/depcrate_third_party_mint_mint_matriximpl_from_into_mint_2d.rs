// Generated macro for impl_from_into_mint_2D (macro)
macro_rules! Depcrate_third_party_mint_mint_matriximpl_from_into_mint_2D {
() => {
// Module: crate::third_party::mint::mint_matrix
// Provides: {"impl_from_into_mint_2D"}
// Dependencies: {}
macro_rules ! impl_from_into_mint_2D (($ (($ NRows : ty , $ NCols : ty) => $ MV : ident { $ ($ component : ident) ,* } [$ SZRows : expr]) ;* $ (;) *) => { $ (impl < T > From < mint ::$ MV < T >> for OMatrix < T , $ NRows , $ NCols > where T : Scalar , DefaultAllocator : Allocator <$ NRows , $ NCols > { # [inline] fn from (m : mint ::$ MV < T >) -> Self { unsafe { let mut res = Matrix :: uninit (<$ NRows >:: name () , <$ NCols >:: name ()) ; let mut ptr = res . data . ptr_mut () ; $ (ptr :: copy_nonoverlapping (& m .$ component . x , ptr as * mut T , $ SZRows) ; ptr = ptr . offset ($ SZRows) ;) * let _ = ptr ; mem :: forget (m) ; res . assume_init () } } } impl < T > Into < mint ::$ MV < T >> for OMatrix < T , $ NRows , $ NCols > where T : Scalar , DefaultAllocator : Allocator <$ NRows , $ NCols > { # [inline] fn into (self) -> mint ::$ MV < T > { unsafe { let mut res : MaybeUninit < mint ::$ MV < T >> = MaybeUninit :: uninit () ; let mut ptr = self . data . ptr () ; $ (ptr :: copy_nonoverlapping (ptr , ptr :: addr_of_mut ! ((* res . as_mut_ptr ()) .$ component) as * mut T , $ SZRows) ; ptr = ptr . offset ($ SZRows) ;) * let _ = ptr ; mem :: forget (self) ; res . assume_init () } } }) * }) ;
};
}
