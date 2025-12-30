// Generated macro for impl_597 (impl)
macro_rules! Depcrate_tableimpl_597 {
() => {
// Module: crate::table
// Provides: {"impl_597"}
// Dependencies: {}
impl < 'a , T > Iterator for IterHash < 'a , T > { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { match self . inner . next () { Some (bucket) => Some (unsafe { bucket . as_ref () }) , None => None , } } fn fold < B , F > (self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . inner . fold (init , | acc , bucket | unsafe { f (acc , bucket . as_ref ()) }) } }
};
}
