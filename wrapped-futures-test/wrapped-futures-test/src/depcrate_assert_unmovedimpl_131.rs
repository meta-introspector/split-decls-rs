// Generated macro for impl_131 (impl)
macro_rules! Depcrate_assert_unmovedimpl_131 {
() => {
// Module: crate::assert_unmoved
// Provides: {"impl_131"}
// Dependencies: {}
impl < T > AssertUnmoved < T > { pub (crate) fn new (inner : T) -> Self { Self { inner , this_addr : 0 } } fn poll_with < 'a , U > (mut self : Pin < & 'a mut Self > , f : impl FnOnce (Pin < & 'a mut T >) -> U) -> U { let cur_this = & * self as * const Self as usize ; if self . this_addr == 0 { * self . as_mut () . project () . this_addr = cur_this ; } else { assert_eq ! (self . this_addr , cur_this , "AssertUnmoved moved between poll calls") ; } f (self . project () . inner) } }
};
}
