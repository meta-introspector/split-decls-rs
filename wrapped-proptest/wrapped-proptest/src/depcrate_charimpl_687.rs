// Generated macro for impl_687 (impl)
macro_rules! Depcrate_charimpl_687 {
() => {
// Module: crate::char
// Provides: {"impl_687"}
// Dependencies: {}
impl ValueTree for CharValueTree { type Value = char ; fn current (& self) -> char { :: core :: char :: from_u32 (self . value . current ()) . expect ("Generated non-char value") } fn simplify (& mut self) -> bool { if self . value . simplify () { self . reposition () ; true } else { false } } fn complicate (& mut self) -> bool { if self . value . complicate () { self . reposition () ; true } else { false } } }
};
}
