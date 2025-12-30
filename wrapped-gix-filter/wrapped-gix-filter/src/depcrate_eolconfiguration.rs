// Generated macro for Configuration (struct)
macro_rules! Depcrate_eolConfiguration {
() => {
// Module: crate::eol
// Provides: {"Configuration"}
// Dependencies: {}
# [doc = " Git Configuration that affects how CRLF conversions are applied."] # [derive (Default , Debug , Copy , Clone)] pub struct Configuration { # [doc = " Corresponds to `core.autocrlf`."] pub auto_crlf : AutoCrlf , # [doc = " Corresponds to `core.eol`, and is `None` if unset or set to `native`, or `Some(<mode>)` respectively."] pub eol : Option < Mode > , }
};
}
