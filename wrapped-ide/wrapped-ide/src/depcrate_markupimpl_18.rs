// Generated macro for impl_18 (impl)
macro_rules! Depcrate_markupimpl_18 {
() => {
// Module: crate::markup
// Provides: {"impl_18"}
// Dependencies: {}
impl Markup { pub fn as_str (& self) -> & str { self . text . as_str () } pub fn fenced_block (contents : impl fmt :: Display) -> Markup { format ! ("```rust\n{contents}\n```") . into () } pub fn fenced_block_text (contents : impl fmt :: Display) -> Markup { format ! ("```text\n{contents}\n```") . into () } }
};
}
