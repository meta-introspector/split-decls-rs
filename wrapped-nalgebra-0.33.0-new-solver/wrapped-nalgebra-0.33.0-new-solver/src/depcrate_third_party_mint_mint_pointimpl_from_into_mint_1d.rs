// Generated macro for impl_from_into_mint_1D (macro)
macro_rules! Depcrate_third_party_mint_mint_pointimpl_from_into_mint_1D {
() => {
// Module: crate::third_party::mint::mint_point
// Provides: {"impl_from_into_mint_1D"}
// Dependencies: {}
macro_rules ! impl_from_into_mint_1D (($ ($ NRows : expr => $ PT : ident , $ VT : ident [$ SZ : expr]) ;* $ (;) *) => { $ (impl < T > From < mint ::$ PT < T >> for Point < T , $ NRows > where T : Scalar { # [inline] fn from (p : mint ::$ PT < T >) -> Self { Self { coords : OVector :: from (mint ::$ VT :: from (p)) , } } } impl < T > Into < mint ::$ PT < T >> for Point < T , $ NRows > where T : Scalar { # [inline] fn into (self) -> mint ::$ PT < T > { let mint_vec : mint ::$ VT < T > = self . coords . into () ; mint ::$ PT :: from (mint_vec) } } impl < T > AsRef < mint ::$ PT < T >> for Point < T , $ NRows > where T : Scalar { # [inline] fn as_ref (& self) -> & mint ::$ PT < T > { unsafe { &* (self . coords . data . ptr () as * const mint ::$ PT < T >) } } } impl < T > AsMut < mint ::$ PT < T >> for Point < T , $ NRows > where T : Scalar { # [inline] fn as_mut (& mut self) -> & mut mint ::$ PT < T > { unsafe { & mut * (self . coords . data . ptr_mut () as * mut mint ::$ PT < T >) } } }) * }) ;
};
}
