// Generated macro for explicit_label (function)
macro_rules! Depcrate_labelexplicit_label {
() => {
// Module: crate::label
// Provides: {"explicit_label"}
// Dependencies: {}
fn explicit_label (explicit : LabelValue) -> Label { match explicit { LabelValue :: Const (explicit) => Label :: Const (quote ! (# explicit)) , LabelValue :: Ident (explicit) => Label :: Ident (explicit) , } }
};
}
