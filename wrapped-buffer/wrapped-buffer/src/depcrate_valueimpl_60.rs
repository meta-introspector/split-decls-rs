// Generated macro for impl_60 (impl)
macro_rules! Depcrate_valueimpl_60 {
() => {
// Module: crate::value
// Provides: {"impl_60"}
// Dependencies: {}
impl < T , const N : usize > BufMut < T , N > { fn push (& mut self , value : T) -> Result < () , Error > { # [cfg (feature = "alloc")] { self . inner . push (value) ; Ok (()) } # [cfg (not (feature = "alloc"))] { self . inner . push (value) } } fn pop (& mut self) -> Option < T > { self . inner . pop () } fn clear (& mut self) { self . inner . clear () } }
};
}
