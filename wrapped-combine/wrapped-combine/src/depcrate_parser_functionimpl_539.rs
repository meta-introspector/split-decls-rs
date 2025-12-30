// Generated macro for impl_539 (impl)
macro_rules! Depcrate_parser_functionimpl_539 {
() => {
// Module: crate::parser::function
// Provides: {"impl_539"}
// Dependencies: {}
impl < E , Input , T > Clone for EnvParser < E , Input , T > where Input : Stream , E : Clone , { fn clone (& self) -> Self { EnvParser { env : self . env . clone () , parser : self . parser , } } }
};
}
