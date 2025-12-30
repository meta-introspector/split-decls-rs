// Generated macro for __impl_iter (macro)
macro_rules! Depcrate_iter__impl_iter {
() => {
// Module: crate::iter
// Provides: {"__impl_iter"}
// Dependencies: {}
# [doc (hidden)] macro_rules ! __impl_iter { (impl <$ ($ lifetime : lifetime ,) ? $ t1 : ident : $ bound1 : ident $ (+ $ bound1_b : ident) ? $ (, $ t2 : ident : $ bound2 : ident $ (+ $ bound2_b : ident) ?) ?> Iterator < Item = $ item : ty > for $ for : ty { ... }) => { impl <$ ($ lifetime ,) ? $ t1 : $ bound1 $ (+ $ bound1_b) ? $ (, $ t2 : $ bound2 $ (+ $ bound2_b) ?) ?> Iterator for $ for { type Item = $ item ; # [inline] # [track_caller] fn next (& mut self) -> Option < Self :: Item > { self . 0 . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } } } }
};
}
