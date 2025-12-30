// Generated macro for DefaultTerminal (type)
macro_rules! Depcrate_initDefaultTerminal {
() => {
// Module: crate::init
// Provides: {"DefaultTerminal"}
// Dependencies: {}
# [doc = " A type alias for the default terminal type."] # [doc = ""] # [doc = " This is a [`Terminal`] using the [`CrosstermBackend`] which writes to [`Stdout`]. This is a"] # [doc = " reasonable default for most applications. To use a different backend or output stream, instead"] # [doc = " use [`Terminal`] and a [backend][`crate::backend`] of your choice directly."] pub type DefaultTerminal = Terminal < CrosstermBackend < Stdout > > ;
};
}
