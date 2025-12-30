// Generated macro for impl_632 (impl)
macro_rules! Depcrate_repeatnimpl_632 {
() => {
// Module: crate::repeatn
// Provides: {"impl_632"}
// Dependencies: {}
impl < A > Iterator for RepeatN < A > where A : Clone , { type Item = A ; fn next (& mut self) -> Option < Self :: Item > { if self . n > 1 { self . n -= 1 ; self . elt . as_ref () . cloned () } else { self . n = 0 ; self . elt . take () } } fn size_hint (& self) -> (usize , Option < usize >) { (self . n , Some (self . n)) } fn fold < B , F > (self , mut init : B , mut f : F) -> B where F : FnMut (B , Self :: Item) -> B , { match self { Self { elt : Some (elt) , n } => { debug_assert ! (n > 0) ; init = (1 .. n) . map (| _ | elt . clone ()) . fold (init , & mut f) ; f (init , elt) } _ => init , } } }
};
}
