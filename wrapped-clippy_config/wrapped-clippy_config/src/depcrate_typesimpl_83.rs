// Generated macro for impl_83 (impl)
macro_rules! Depcrate_typesimpl_83 {
() => {
// Module: crate::types
// Provides: {"impl_83"}
// Dependencies: {}
impl DisallowedPathEnum { pub fn path (& self) -> & str { let (Self :: Simple (path) | Self :: WithReason { path , .. }) = self ; path } fn reason (& self) -> Option < & str > { match & self { Self :: WithReason { reason , .. } => reason . as_deref () , Self :: Simple (_) => None , } } fn replacement (& self) -> Option < & str > { match & self { Self :: WithReason { replacement , .. } => replacement . as_deref () , Self :: Simple (_) => None , } } fn allow_invalid (& self) -> bool { match & self { Self :: WithReason { allow_invalid , .. } => allow_invalid . unwrap_or_default () , Self :: Simple (_) => false , } } }
};
}
