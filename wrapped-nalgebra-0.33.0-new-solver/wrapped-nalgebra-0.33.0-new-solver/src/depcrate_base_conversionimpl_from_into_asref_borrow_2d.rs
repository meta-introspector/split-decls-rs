// Generated macro for impl_from_into_asref_borrow_2D (macro)
macro_rules! Depcrate_base_conversionimpl_from_into_asref_borrow_2D {
() => {
// Module: crate::base::conversion
// Provides: {"impl_from_into_asref_borrow_2D"}
// Dependencies: {}
macro_rules ! impl_from_into_asref_borrow_2D ((($ NRows : ty , $ NCols : ty) => ($ SZRows : expr , $ SZCols : expr) ; $ Ref : ident .$ ref : ident () , $ Mut : ident .$ mut : ident ()) => { impl < T : Scalar , S > $ Ref < [[T ; $ SZRows] ; $ SZCols] > for Matrix < T , $ NRows , $ NCols , S > where S : RawStorage < T , $ NRows , $ NCols > + IsContiguous { # [inline] fn $ ref (& self) -> & [[T ; $ SZRows] ; $ SZCols] { unsafe { &* (self . data . ptr () as * const [[T ; $ SZRows] ; $ SZCols]) } } } impl < T : Scalar , S > $ Mut < [[T ; $ SZRows] ; $ SZCols] > for Matrix < T , $ NRows , $ NCols , S > where S : RawStorageMut < T , $ NRows , $ NCols > + IsContiguous { # [inline] fn $ mut (& mut self) -> & mut [[T ; $ SZRows] ; $ SZCols] { unsafe { & mut * (self . data . ptr_mut () as * mut [[T ; $ SZRows] ; $ SZCols]) } } } } ; ($ (($ NRows : ty , $ NCols : ty) => ($ SZRows : expr , $ SZCols : expr)) ;* $ (;) *) => { $ (impl_from_into_asref_borrow_2D ! (($ NRows , $ NCols) => ($ SZRows , $ SZCols) ; AsRef . as_ref () , AsMut . as_mut ()) ; impl_from_into_asref_borrow_2D ! (($ NRows , $ NCols) => ($ SZRows , $ SZCols) ; Borrow . borrow () , BorrowMut . borrow_mut ()) ;) * }) ;
};
}
