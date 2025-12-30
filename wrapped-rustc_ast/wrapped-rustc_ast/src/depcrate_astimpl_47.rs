// Generated macro for impl_47 (impl)
macro_rules! Depcrate_astimpl_47 {
() => {
// Module: crate::ast
// Provides: {"impl_47"}
// Dependencies: {}
impl ParenthesizedArgs { pub fn as_angle_bracketed_args (& self) -> AngleBracketedArgs { let args = self . inputs . iter () . cloned () . map (| input | AngleBracketedArg :: Arg (GenericArg :: Type (input))) . collect () ; AngleBracketedArgs { span : self . inputs_span , args } } }
};
}
