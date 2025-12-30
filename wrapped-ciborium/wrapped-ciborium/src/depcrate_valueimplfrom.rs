// Generated macro for implfrom (macro)
macro_rules! Depcrate_valueimplfrom {
() => {
// Module: crate::value
// Provides: {"implfrom"}
// Dependencies: {}
macro_rules ! implfrom { ($ ($ v : ident ($ t : ty)) ,+ $ (,) ?) => { $ (impl From <$ t > for Value { # [inline] fn from (value : $ t) -> Self { Self ::$ v (value . into ()) } }) + } ; }
};
}
