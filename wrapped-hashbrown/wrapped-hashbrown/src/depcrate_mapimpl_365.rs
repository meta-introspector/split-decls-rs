// Generated macro for impl_365 (impl)
macro_rules! Depcrate_mapimpl_365 {
() => {
// Module: crate::map
// Provides: {"impl_365"}
// Dependencies: {}
impl < 'a , K , V > Iterator for Iter < 'a , K , V > { type Item = (& 'a K , & 'a V) ; # [cfg_attr (feature = "inline-more" , inline)] fn next (& mut self) -> Option < (& 'a K , & 'a V) > { match self . inner . next () { Some (x) => unsafe { let r = x . as_ref () ; Some ((& r . 0 , & r . 1)) } , None => None , } } # [cfg_attr (feature = "inline-more" , inline)] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } # [cfg_attr (feature = "inline-more" , inline)] fn fold < B , F > (self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . inner . fold (init , | acc , x | unsafe { let (k , v) = x . as_ref () ; f (acc , (k , v)) }) } }
};
}
