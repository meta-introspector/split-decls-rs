// Generated macro for impl_269 (impl)
macro_rules! Depcrate_query_engine_updateimpl_269 {
() => {
// Module: crate::query::engine::update
// Provides: {"impl_269"}
// Dependencies: {}
impl FileMode { pub fn as_str (& self) -> & 'static str { use FileMode :: * ; match self { Added => "+" , Removed => "-" , Modified => "Δ" , Rename => "➡" , Copy => "⏸" , } } pub fn from_usize (mode : usize) -> Option < Self > { use FileMode :: * ; match mode { 1 => Added , 2 => Removed , 3 => Modified , 4 => Rename , 5 => Copy , _ => return None , } . into () } }
};
}
