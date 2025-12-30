// Generated macro for impl_101 (impl)
macro_rules! Depcrate_astimpl_101 {
() => {
// Module: crate::ast
// Provides: {"impl_101"}
// Dependencies: {}
impl LocalKind { pub fn init (& self) -> Option < & Expr > { match self { Self :: Decl => None , Self :: Init (i) | Self :: InitElse (i , _) => Some (i) , } } pub fn init_else_opt (& self) -> Option < (& Expr , Option < & Block >) > { match self { Self :: Decl => None , Self :: Init (init) => Some ((init , None)) , Self :: InitElse (init , els) => Some ((init , Some (els))) , } } }
};
}
