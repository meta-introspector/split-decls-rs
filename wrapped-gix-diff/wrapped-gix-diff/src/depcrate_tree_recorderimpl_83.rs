// Generated macro for impl_83 (impl)
macro_rules! Depcrate_tree_recorderimpl_83 {
() => {
// Module: crate::tree::recorder
// Provides: {"impl_83"}
// Dependencies: {}
impl Recorder { fn pop_element (& mut self) { if let Some (pos) = self . path . rfind_byte (b'/') { self . path . resize (pos , 0) ; } else { self . path . clear () ; } } fn push_element (& mut self , name : & BStr) { if name . is_empty () { return ; } if ! self . path . is_empty () { self . path . push (b'/') ; } self . path . push_str (name) ; } }
};
}
