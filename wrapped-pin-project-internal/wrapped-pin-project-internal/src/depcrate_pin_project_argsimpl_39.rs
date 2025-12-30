// Generated macro for impl_39 (impl)
macro_rules! Depcrate_pin_project_argsimpl_39 {
() => {
// Module: crate::pin_project::args
// Provides: {"impl_39"}
// Dependencies: {}
impl ProjReplace { # [doc = " Return the span of this argument."] pub (super) fn span (& self) -> Option < Span > { match self { Self :: None => None , Self :: Named { span , .. } | Self :: Unnamed { span , .. } => Some (* span) , } } pub (super) fn ident (& self) -> Option < & Ident > { if let Self :: Named { ident , .. } = self { Some (ident) } else { None } } }
};
}
