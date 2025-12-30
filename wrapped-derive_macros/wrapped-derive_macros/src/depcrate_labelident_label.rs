// Generated macro for ident_label (function)
macro_rules! Depcrate_labelident_label {
() => {
// Module: crate::label
// Provides: {"ident_label"}
// Dependencies: {}
fn ident_label (ident : & Ident) -> Label { Label :: Implicit ({ let ident = ident . unraw () . to_string () ; quote ! (# ident) }) }
};
}
