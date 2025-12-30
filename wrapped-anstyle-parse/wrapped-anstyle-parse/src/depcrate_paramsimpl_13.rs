// Generated macro for impl_13 (impl)
macro_rules! Depcrate_paramsimpl_13 {
() => {
// Module: crate::params
// Provides: {"impl_13"}
// Dependencies: {}
impl < 'a > Iterator for ParamsIter < 'a > { type Item = & 'a [u16] ; fn next (& mut self) -> Option < Self :: Item > { if self . index >= self . params . len () { return None ; } let num_subparams = self . params . subparams [self . index] ; let param = & self . params . params [self . index .. self . index + num_subparams as usize] ; self . index += num_subparams as usize ; Some (param) } fn size_hint (& self) -> (usize , Option < usize >) { let remaining = self . params . len () - self . index ; (remaining , Some (remaining)) } }
};
}
