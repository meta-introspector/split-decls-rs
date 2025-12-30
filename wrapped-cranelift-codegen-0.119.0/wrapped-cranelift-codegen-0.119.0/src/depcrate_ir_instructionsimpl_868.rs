// Generated macro for impl_868 (impl)
macro_rules! Depcrate_ir_instructionsimpl_868 {
() => {
// Module: crate::ir::instructions
// Provides: {"impl_868"}
// Dependencies: {}
impl Display for VariableArgs { fn fmt (& self , fmt : & mut Formatter) -> fmt :: Result { for (i , val) in self . 0 . iter () . enumerate () { if i == 0 { write ! (fmt , "{val}") ? ; } else { write ! (fmt , ", {val}") ? ; } } Ok (()) } }
};
}
