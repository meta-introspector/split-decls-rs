// Generated macro for impl_102 (impl)
macro_rules! Depcrate_eventimpl_102 {
() => {
// Module: crate::event
// Provides: {"impl_102"}
// Dependencies: {}
impl From < KeyCode > for KeyEvent { fn from (code : KeyCode) -> Self { KeyEvent { code , modifiers : KeyModifiers :: empty () , kind : KeyEventKind :: Press , state : KeyEventState :: empty () , } } }
};
}
