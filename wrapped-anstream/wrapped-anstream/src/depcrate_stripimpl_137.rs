// Generated macro for impl_137 (impl)
macro_rules! Depcrate_stripimpl_137 {
() => {
// Module: crate::strip
// Provides: {"impl_137"}
// Dependencies: {}
impl < S > StripStream < S > where S : std :: io :: Write , S : IsTerminal , { # [doc = " Returns `true` if the descriptor/handle refers to a terminal/tty."] # [inline] pub fn is_terminal (& self) -> bool { self . raw . is_terminal () } }
};
}
