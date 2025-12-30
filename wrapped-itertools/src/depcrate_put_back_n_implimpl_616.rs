// Generated macro for impl_616 (impl)
macro_rules! Depcrate_put_back_n_implimpl_616 {
() => {
// Module: crate::put_back_n_impl
// Provides: {"impl_616"}
// Dependencies: {}
impl < I : Iterator > Iterator for PutBackN < I > { type Item = I :: Item ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . top . pop () . or_else (| | self . iter . next ()) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { size_hint :: add_scalar (self . iter . size_hint () , self . top . len ()) } fn fold < B , F > (self , mut init : B , mut f : F) -> B where F : FnMut (B , Self :: Item) -> B , { init = self . top . into_iter () . rfold (init , & mut f) ; self . iter . fold (init , f) } }
};
}
