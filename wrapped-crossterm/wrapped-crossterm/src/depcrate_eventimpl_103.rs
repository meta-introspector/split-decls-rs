// Generated macro for impl_103 (impl)
macro_rules! Depcrate_eventimpl_103 {
() => {
// Module: crate::event
// Provides: {"impl_103"}
// Dependencies: {}
impl PartialEq for KeyEvent { fn eq (& self , other : & KeyEvent) -> bool { let KeyEvent { code : lhs_code , modifiers : lhs_modifiers , kind : lhs_kind , state : lhs_state , } = self . normalize_case () ; let KeyEvent { code : rhs_code , modifiers : rhs_modifiers , kind : rhs_kind , state : rhs_state , } = other . normalize_case () ; (lhs_code == rhs_code) && (lhs_modifiers == rhs_modifiers) && (lhs_kind == rhs_kind) && (lhs_state == rhs_state) } }
};
}
