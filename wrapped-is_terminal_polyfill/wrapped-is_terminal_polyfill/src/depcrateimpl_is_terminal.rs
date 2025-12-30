// Generated macro for impl_is_terminal (macro)
macro_rules! Depcrateimpl_is_terminal {
() => {
// Module: crate
// Provides: {"impl_is_terminal"}
// Dependencies: {}
macro_rules ! impl_is_terminal { ($ ($ t : ty) ,*$ (,) ?) => { $ (impl sealed :: Sealed for $ t { } impl IsTerminal for $ t { # [inline] fn is_terminal (& self) -> bool { std :: io :: IsTerminal :: is_terminal (self) } }) * } }
};
}
