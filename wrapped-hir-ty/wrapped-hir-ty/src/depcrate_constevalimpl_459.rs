// Generated macro for impl_459 (impl)
macro_rules! Depcrate_constevalimpl_459 {
() => {
// Module: crate::consteval
// Provides: {"impl_459"}
// Dependencies: {}
impl ConstEvalError < '_ > { pub fn pretty_print (& self , f : & mut String , db : & dyn HirDatabase , span_formatter : impl Fn (span :: FileId , span :: TextRange) -> String , display_target : DisplayTarget ,) -> std :: result :: Result < () , std :: fmt :: Error > { match self { ConstEvalError :: MirLowerError (e) => { e . pretty_print (f , db , span_formatter , display_target) } ConstEvalError :: MirEvalError (e) => { e . pretty_print (f , db , span_formatter , display_target) } } } }
};
}
