// Generated macro for impl_239 (impl)
macro_rules! Depcrate_danglingimpl_239 {
() => {
// Module: crate::dangling
// Provides: {"impl_239"}
// Dependencies: {}
impl DanglingPointerSearcher < '_ , '_ > { fn with_inside_call_args < R > (& mut self , inside_call_args : bool , callback : impl FnOnce (& mut Self) -> R ,) -> R { let old = core :: mem :: replace (& mut self . inside_call_args , inside_call_args) ; let result = callback (self) ; self . inside_call_args = old ; result } }
};
}
