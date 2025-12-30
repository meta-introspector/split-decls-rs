// Generated macro for impl_172 (impl)
macro_rules! Depcrate_iter_plumbingimpl_172 {
() => {
// Module: crate::iter::plumbing
// Provides: {"impl_172"}
// Dependencies: {}
impl Splitter { # [inline] fn new () -> Splitter { Splitter { splits : crate :: current_num_threads () , } } # [inline] fn try_split (& mut self , stolen : bool) -> bool { let Splitter { splits } = * self ; if stolen { self . splits = Ord :: max (crate :: current_num_threads () , self . splits / 2) ; true } else if splits > 0 { self . splits /= 2 ; true } else { false } } }
};
}
