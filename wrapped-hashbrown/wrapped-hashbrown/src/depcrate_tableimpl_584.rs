// Generated macro for impl_584 (impl)
macro_rules! Depcrate_tableimpl_584 {
() => {
// Module: crate::table
// Provides: {"impl_584"}
// Dependencies: {}
impl < 'a , T > Iterator for Iter < 'a , T > { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { match self . inner . next () { Some (bucket) => Some (unsafe { bucket . as_ref () }) , None => None , } } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } fn fold < B , F > (self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . inner . fold (init , | acc , bucket | unsafe { f (acc , bucket . as_ref ()) }) } }
};
}
