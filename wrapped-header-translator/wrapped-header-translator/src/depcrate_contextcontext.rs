// Generated macro for Context (struct)
macro_rules! Depcrate_contextContext {
() => {
// Module: crate::context
// Provides: {"Context"}
// Dependencies: {}
pub struct Context < 'config > { config : & 'config Config , pub macro_invocations : HashMap < MacroLocation , MacroEntity > , pub ident_mapping : HashMap < String , Expr > , pub current_library : & 'config str , }
};
}
