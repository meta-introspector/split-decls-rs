// Generated macro for impl_387 (impl)
macro_rules! Depcrate_mapimpl_387 {
() => {
// Module: crate::map
// Provides: {"impl_387"}
// Dependencies: {}
impl < 'a , K , V > Iterator for ValuesMut < 'a , K , V > { type Item = & 'a mut V ; # [cfg_attr (feature = "inline-more" , inline)] fn next (& mut self) -> Option < & 'a mut V > { match self . inner . next () { Some ((_ , v)) => Some (v) , None => None , } } # [cfg_attr (feature = "inline-more" , inline)] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } # [cfg_attr (feature = "inline-more" , inline)] fn fold < B , F > (self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . inner . fold (init , | acc , (_ , v) | f (acc , v)) } }
};
}
