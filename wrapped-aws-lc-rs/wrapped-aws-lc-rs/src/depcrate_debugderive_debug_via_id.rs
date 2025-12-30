// Generated macro for derive_debug_via_id (macro)
macro_rules! Depcrate_debugderive_debug_via_id {
() => {
// Module: crate::debug
// Provides: {"derive_debug_via_id"}
// Dependencies: {}
macro_rules ! derive_debug_via_id { ($ typename : ident) => { impl :: core :: fmt :: Debug for $ typename { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> Result < () , :: core :: fmt :: Error > { :: core :: fmt :: Debug :: fmt (& self . id , f) } } } ; }
};
}
