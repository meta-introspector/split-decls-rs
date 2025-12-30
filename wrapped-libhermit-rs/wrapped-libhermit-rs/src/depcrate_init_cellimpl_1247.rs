// Generated macro for impl_1247 (impl)
macro_rules! Depcrate_init_cellimpl_1247 {
() => {
// Module: crate::init_cell
// Provides: {"impl_1247"}
// Dependencies: {}
impl < T > InitCell < T > { pub const fn new (val : T) -> Self { Self { init : SpinMutex :: new (Some (val)) , once : OnceCell :: new () , } } pub fn with (& self , f : impl FnOnce (Option < & mut T >)) { let mut guard = self . init . lock () ; f ((* guard) . as_mut ()) ; } pub fn get (& self) -> Option < & T > { self . once . get () } pub fn finalize (& self) -> & T { self . once . get_or_init (| | self . init . lock () . take () . unwrap ()) } }
};
}
