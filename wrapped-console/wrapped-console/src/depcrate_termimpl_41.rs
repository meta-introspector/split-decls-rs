// Generated macro for impl_41 (impl)
macro_rules! Depcrate_termimpl_41 {
() => {
// Module: crate::term
// Provides: {"impl_41"}
// Dependencies: {}
impl TermFeatures < '_ > { # [doc = " Check if this is a real user attended terminal (`isatty`)"] # [inline] pub fn is_attended (& self) -> bool { is_a_terminal (self . 0) } # [doc = " Check if colors are supported by this terminal."] # [doc = ""] # [doc = " This does not check if colors are enabled.  Currently all terminals"] # [doc = " are considered to support colors"] # [inline] pub fn colors_supported (& self) -> bool { is_a_color_terminal (self . 0) } # [doc = " Check if true colors are supported by this terminal."] pub fn true_colors_supported (& self) -> bool { is_a_true_color_terminal (self . 0) } # [doc = " Check if this terminal is an msys terminal."] # [doc = ""] # [doc = " This is sometimes useful to disable features that are known to not"] # [doc = " work on msys terminals or require special handling."] # [inline] pub fn is_msys_tty (& self) -> bool { # [cfg (windows)] { msys_tty_on (self . 0) } # [cfg (not (windows))] { false } } # [doc = " Check if this terminal wants emojis."] # [inline] pub fn wants_emoji (& self) -> bool { self . is_attended () && wants_emoji () } # [doc = " Return the family of the terminal."] # [inline] pub fn family (& self) -> TermFamily { if ! self . is_attended () { return TermFamily :: File ; } # [cfg (windows)] { TermFamily :: WindowsConsole } # [cfg (all (unix , not (target_arch = "wasm32")))] { TermFamily :: UnixTerm } # [cfg (target_arch = "wasm32")] { TermFamily :: Dummy } } }
};
}
