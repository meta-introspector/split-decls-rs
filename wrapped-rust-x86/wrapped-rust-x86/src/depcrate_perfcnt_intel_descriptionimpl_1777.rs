// Generated macro for impl_1777 (impl)
macro_rules! Depcrate_perfcnt_intel_descriptionimpl_1777 {
() => {
// Module: crate::perfcnt::intel::description
// Provides: {"impl_1777"}
// Dependencies: {}
impl fmt :: Debug for Counter { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { Counter :: Fixed (a) => write ! (f , "Counter::Fixed({})" , a) , Counter :: Programmable (a) => write ! (f , "Counter::Programmable({})" , a) , } } }
};
}
