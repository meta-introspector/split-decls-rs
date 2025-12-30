// Generated macro for impl_196 (impl)
macro_rules! Depcrate_astimpl_196 {
() => {
// Module: crate::ast
// Provides: {"impl_196"}
// Dependencies: {}
impl BoundConstness { pub fn as_str (self) -> & 'static str { match self { Self :: Never => "" , Self :: Always (_) => "const" , Self :: Maybe (_) => "[const]" , } } }
};
}
