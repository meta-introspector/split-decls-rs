// Generated macro for impl_6 (impl)
macro_rules! Depcrate_counterimpl_6 {
() => {
// Module: crate::counter
// Provides: {"impl_6"}
// Dependencies: {}
impl Counter { # [doc = " Create `Counter` instance with max value."] pub fn new (capacity : usize) -> Self { Counter (Rc :: new (CounterInner { capacity , count : Cell :: new (0) , task : LocalWaker :: new () , })) } # [doc = " Create new counter guard, incrementing the counter."] # [inline] pub fn get (& self) -> CounterGuard { CounterGuard :: new (self . 0 . clone ()) } # [doc = " Returns true if counter is below capacity. Otherwise, register to wake task when it is."] # [inline] pub fn available (& self , cx : & task :: Context < '_ >) -> bool { self . 0 . available (cx) } # [doc = " Get total number of acquired guards."] # [inline] pub fn total (& self) -> usize { self . 0 . count . get () } }
};
}
