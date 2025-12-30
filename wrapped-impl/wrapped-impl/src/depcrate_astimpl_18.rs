// Generated macro for impl_18 (impl)
macro_rules! Depcrate_astimpl_18 {
() => {
// Module: crate::ast
// Provides: {"impl_18"}
// Dependencies: {}
impl < 'a > Variant < 'a > { fn from_syn (node : & 'a syn :: Variant , scope : & ParamsInScope < 'a >) -> Result < Self > { let attrs = attr :: get (& node . attrs) ? ; Ok (Variant { original : node , attrs , ident : node . ident . clone () , fields : Field :: multiple_from_syn (& node . fields , scope) ? , }) } }
};
}
