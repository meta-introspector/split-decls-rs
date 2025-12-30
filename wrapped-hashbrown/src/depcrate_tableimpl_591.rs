// Generated macro for impl_591 (impl)
macro_rules! Depcrate_tableimpl_591 {
() => {
// Module: crate::table
// Provides: {"impl_591"}
// Dependencies: {}
impl < 'a , T > Iterator for IterMut < 'a , T > { type Item = & 'a mut T ; fn next (& mut self) -> Option < Self :: Item > { match self . inner . next () { Some (bucket) => Some (unsafe { bucket . as_mut () }) , None => None , } } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } fn fold < B , F > (self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . inner . fold (init , | acc , bucket | unsafe { f (acc , bucket . as_mut ()) }) } }
};
}
