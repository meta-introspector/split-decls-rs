// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
# [doc = " A button with a label that can be themed."] impl < 'a > Button < 'a > { pub fn new < T : Into < Line < 'a > > > (label : T) -> Self { Button { label : label . into () , theme : BLUE , state : State :: Normal , } } pub const fn theme (mut self , theme : Theme) -> Self { self . theme = theme ; self } pub const fn state (mut self , state : State) -> Self { self . state = state ; self } }
};
}
