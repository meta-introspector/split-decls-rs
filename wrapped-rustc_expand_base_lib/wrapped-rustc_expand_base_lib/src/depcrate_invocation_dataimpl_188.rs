// Generated macro for impl_188 (impl)
macro_rules! Depcrate_invocation_dataimpl_188 {
() => {
// Module: crate::invocation_data
// Provides: {"impl_188"}
// Dependencies: {}
impl Invocation { # [doc = " Returns the `Span` (source code location) of this macro invocation."] pub fn span (& self) -> Span { match & self . kind { InvocationKind :: Bang { span , .. } => * span , InvocationKind :: Attr { attr , .. } => attr . span , InvocationKind :: Derive { path , .. } => path . span , InvocationKind :: GlobDelegation { item , .. } => item . span , } } # [doc = " Returns a mutable reference to the `Span` of this macro invocation."] pub fn span_mut (& mut self) -> & mut Span { match & mut self . kind { InvocationKind :: Bang { span , .. } => span , InvocationKind :: Attr { attr , .. } => & mut attr . span , InvocationKind :: Derive { path , .. } => & mut path . span , InvocationKind :: GlobDelegation { item , .. } => & mut item . span , } } }
};
}
