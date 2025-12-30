// Generated macro for Term (struct)
macro_rules! Depcrate_termTerm {
() => {
// Module: crate::term
// Provides: {"Term"}
// Dependencies: {}
# [doc = " Abstraction around a terminal."] # [doc = ""] # [doc = " A terminal can be cloned.  If a buffer is used it's shared across all"] # [doc = " clones which means it largely acts as a handle."] # [derive (Clone , Debug)] pub struct Term { inner : Arc < TermInner > , pub (crate) is_msys_tty : bool , pub (crate) is_tty : bool , }
};
}
