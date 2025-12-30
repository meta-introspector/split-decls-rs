// Generated macro for impl_22 (impl)
macro_rules! Depcrate_appimpl_22 {
() => {
// Module: crate::app
// Provides: {"impl_22"}
// Dependencies: {}
impl Tab { fn next (self) -> Self { let current_index = self as usize ; let next_index = current_index . saturating_add (1) ; Self :: from_repr (next_index) . unwrap_or (self) } fn prev (self) -> Self { let current_index = self as usize ; let prev_index = current_index . saturating_sub (1) ; Self :: from_repr (prev_index) . unwrap_or (self) } fn title (self) -> String { match self { Self :: About => String :: new () , tab => format ! (" {tab} ") , } } }
};
}
