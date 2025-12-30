// Generated macro for impl_923 (impl)
macro_rules! Depcrate_patimpl_923 {
() => {
// Module: crate::pat
// Provides: {"impl_923"}
// Dependencies: {}
impl MutblCap { # [must_use] fn cap_to_weakly_not (self , span : Option < Span >) -> Self { match self { MutblCap :: Not => MutblCap :: Not , _ => MutblCap :: WeaklyNot (span) , } } # [must_use] fn as_mutbl (self) -> Mutability { match self { MutblCap :: Not | MutblCap :: WeaklyNot (_) => Mutability :: Not , MutblCap :: Mut => Mutability :: Mut , } } }
};
}
