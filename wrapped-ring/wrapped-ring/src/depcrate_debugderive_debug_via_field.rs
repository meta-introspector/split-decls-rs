// Generated macro for derive_debug_via_field (macro)
macro_rules! Depcrate_debugderive_debug_via_field {
() => {
// Module: crate::debug
// Provides: {"derive_debug_via_field"}
// Dependencies: {}
macro_rules ! derive_debug_via_field { ($ type : ty , $ field : ident) => { derive_debug_via_field ! ($ type , stringify ! ($ type) , $ field) ; } ; ($ type : ty , $ typename : expr , $ field : ident) => { impl :: core :: fmt :: Debug for $ type { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> Result < () , :: core :: fmt :: Error > { f . debug_struct ($ typename) . field (stringify ! ($ field) , & self .$ field) . finish () } } } ; }
};
}
