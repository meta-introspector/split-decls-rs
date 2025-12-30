// Generated macro for impl_255 (impl)
macro_rules! Depcrate_combinationsimpl_255 {
() => {
// Module: crate::combinations
// Provides: {"impl_255"}
// Dependencies: {}
impl < I : Iterator > Combinations < I > { # [doc = " Resets this `Combinations` back to an initial state for combinations of length"] # [doc = " `k` over the same pool data source. If `k` is larger than the current length"] # [doc = " of the data pool an attempt is made to prefill the pool so that it holds `k`"] # [doc = " elements."] pub (crate) fn reset (& mut self , k : usize) { self . first = true ; if k < self . indices . len () { self . indices . truncate (k) ; for i in 0 .. k { self . indices [i] = i ; } } else { for i in 0 .. self . indices . len () { self . indices [i] = i ; } self . indices . extend (self . indices . len () .. k) ; self . pool . prefill (k) ; } } }
};
}
