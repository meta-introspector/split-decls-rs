// Generated macro for CustomDirectiveFactory (trait)
macro_rules! Depcrate_custom_directiveCustomDirectiveFactory {
() => {
// Module: crate::custom_directive
// Provides: {"CustomDirectiveFactory"}
// Dependencies: {}
# [doc (hidden)] pub trait CustomDirectiveFactory : Send + Sync + 'static { fn name (& self) -> Cow < 'static , str > ; fn register (& self , registry : & mut Registry) ; fn create (& self , ctx : & ContextDirective < '_ > , directive : & Directive ,) -> ServerResult < Box < dyn CustomDirective > > ; }
};
}
