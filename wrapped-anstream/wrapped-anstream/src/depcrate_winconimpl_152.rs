// Generated macro for impl_152 (impl)
macro_rules! Depcrate_winconimpl_152 {
() => {
// Module: crate::wincon
// Provides: {"impl_152"}
// Dependencies: {}
impl < S > WinconStream < S > where S : anstyle_wincon :: WinconStream , S : IsTerminal , { # [doc = " Returns `true` if the descriptor/handle refers to a terminal/tty."] # [inline] pub fn is_terminal (& self) -> bool { self . raw . is_terminal () } }
};
}
