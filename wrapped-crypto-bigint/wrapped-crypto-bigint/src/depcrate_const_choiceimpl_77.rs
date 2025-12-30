// Generated macro for impl_77 (impl)
macro_rules! Depcrate_const_choiceimpl_77 {
() => {
// Module: crate::const_choice
// Provides: {"impl_77"}
// Dependencies: {}
impl From < Choice > for ConstChoice { # [inline] fn from (choice : Choice) -> Self { ConstChoice :: from_word_lsb (choice . unwrap_u8 () as Word) } }
};
}
