// Generated macro for impl_101 (impl)
macro_rules! Depcrate_stackimpl_101 {
() => {
// Module: crate::stack
// Provides: {"impl_101"}
// Dependencies: {}
impl Error for StackError { fn source (& self) -> Option < & (dyn Error + 'static) > { match * self { StackError :: ExceedsMaximumSize (_) => None , StackError :: IoError (ref e) => Some (e) , } } }
};
}
