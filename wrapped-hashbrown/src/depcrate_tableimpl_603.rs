// Generated macro for impl_603 (impl)
macro_rules! Depcrate_tableimpl_603 {
() => {
// Module: crate::table
// Provides: {"impl_603"}
// Dependencies: {}
impl < 'a , T > Iterator for IterHashMut < 'a , T > { type Item = & 'a mut T ; fn next (& mut self) -> Option < Self :: Item > { match self . inner . next () { Some (bucket) => Some (unsafe { bucket . as_mut () }) , None => None , } } fn fold < B , F > (self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . inner . fold (init , | acc , bucket | unsafe { f (acc , bucket . as_mut ()) }) } }
};
}
