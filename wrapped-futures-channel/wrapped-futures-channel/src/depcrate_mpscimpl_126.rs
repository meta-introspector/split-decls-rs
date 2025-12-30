// Generated macro for impl_126 (impl)
macro_rules! Depcrate_mpscimpl_126 {
() => {
// Module: crate::mpsc
// Provides: {"impl_126"}
// Dependencies: {}
impl State { fn is_closed (& self) -> bool { ! self . is_open && self . num_messages == 0 } fn size_hint (& self) -> (usize , Option < usize >) { if self . is_open { (self . num_messages , None) } else { (self . num_messages , Some (self . num_messages)) } } }
};
}
