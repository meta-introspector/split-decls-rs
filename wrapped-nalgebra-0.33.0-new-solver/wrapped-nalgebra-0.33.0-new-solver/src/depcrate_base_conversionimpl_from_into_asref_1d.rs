// Generated macro for impl_from_into_asref_1D (macro)
macro_rules! Depcrate_base_conversionimpl_from_into_asref_1D {
() => {
// Module: crate::base::conversion
// Provides: {"impl_from_into_asref_1D"}
// Dependencies: {}
macro_rules ! impl_from_into_asref_1D (($ (($ NRows : ident , $ NCols : ident) => $ SZ : expr) ;* $ (;) *) => { $ (impl < T , S > AsRef < [T ; $ SZ] > for Matrix < T , $ NRows , $ NCols , S > where T : Scalar , S : RawStorage < T , $ NRows , $ NCols > + IsContiguous { # [inline] fn as_ref (& self) -> & [T ; $ SZ] { unsafe { &* (self . data . ptr () as * const [T ; $ SZ]) } } } impl < T , S > AsMut < [T ; $ SZ] > for Matrix < T , $ NRows , $ NCols , S > where T : Scalar , S : RawStorageMut < T , $ NRows , $ NCols > + IsContiguous { # [inline] fn as_mut (& mut self) -> & mut [T ; $ SZ] { unsafe { & mut * (self . data . ptr_mut () as * mut [T ; $ SZ]) } } }) * }) ;
};
}
