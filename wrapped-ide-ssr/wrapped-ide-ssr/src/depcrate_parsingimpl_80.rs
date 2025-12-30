// Generated macro for impl_80 (impl)
macro_rules! Depcrate_parsingimpl_80 {
() => {
// Module: crate::parsing
// Provides: {"impl_80"}
// Dependencies: {}
impl NodeKind { fn from (name : & SmolStr) -> Result < NodeKind , SsrError > { Ok (match name . as_str () { "literal" => NodeKind :: Literal , _ => bail ! ("Unknown node kind '{}'" , name) , }) } }
};
}
