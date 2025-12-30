// Generated macro for impl_519 (impl)
macro_rules! Depcrate_setimpl_519 {
() => {
// Module: crate::set
// Provides: {"impl_519"}
// Dependencies: {}
impl < K , A : Allocator > Iterator for IntoIter < K , A > { type Item = K ; # [cfg_attr (feature = "inline-more" , inline)] fn next (& mut self) -> Option < K > { match self . iter . next () { Some ((k , _)) => Some (k) , None => None , } } # [cfg_attr (feature = "inline-more" , inline)] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } # [cfg_attr (feature = "inline-more" , inline)] fn fold < B , F > (self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . iter . fold (init , | acc , (k , ()) | f (acc , k)) } }
};
}
