// Generated macro for impl_148 (impl)
macro_rules! Depcrate_stateimpl_148 {
() => {
// Module: crate::state
// Provides: {"impl_148"}
// Dependencies: {}
impl State { pub (crate) fn new () -> Self { Self { state : 0 } } pub (crate) fn reset (& mut self) { self . state = LIT_LIT ; } pub (crate) fn get (& self) -> u8 { self . state } pub (crate) fn set (& mut self , other : State) { self . state = other . state ; } pub (crate) fn update_literal (& mut self) { if self . state <= SHORTREP_LIT_LIT { self . state = LIT_LIT ; } else if self . state <= LIT_SHORTREP { self . state -= 3 ; } else { self . state -= 6 ; } } pub (crate) fn update_match (& mut self) { self . state = if self . state < LIT_STATES { LIT_MATCH } else { NONLIT_MATCH } ; } pub (crate) fn update_long_rep (& mut self) { self . state = if self . state < LIT_STATES { LIT_LONGREP } else { NONLIT_REP } ; } pub (crate) fn update_short_rep (& mut self) { self . state = if self . state < LIT_STATES { LIT_SHORTREP } else { NONLIT_REP } ; } pub (crate) fn is_literal (& self) -> bool { self . state < LIT_STATES } }
};
}
