// Generated macro for impl_8 (impl)
macro_rules! Depcrate_stackimpl_8 {
() => {
// Module: crate::stack
// Provides: {"impl_8"}
// Dependencies: {}
impl < const CAPACITY : usize > std :: ops :: Deref for StackString < CAPACITY > { type Target = str ; # [inline] fn deref (& self) -> & str { self . as_str () } }
};
}
