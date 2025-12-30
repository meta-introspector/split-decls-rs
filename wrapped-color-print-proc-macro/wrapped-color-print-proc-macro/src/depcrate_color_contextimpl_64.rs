// Generated macro for impl_64 (impl)
macro_rules! Depcrate_color_contextimpl_64 {
() => {
// Module: crate::color_context
// Provides: {"impl_64"}
// Dependencies: {}
impl ChangeSet { # [doc = " Checks if there is nothing to change (used to detect the `</>` tag)."] pub fn is_void (& self) -> bool { and ! (self . foreground . is_none () , self . background . is_none () , ! self . bold , ! self . dim , ! self . underline , ! self . italics , ! self . blink , ! self . strike , ! self . reverse , ! self . conceal ,) } }
};
}
