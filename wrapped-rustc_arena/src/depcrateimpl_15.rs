// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
impl < T > Default for TypedArena < T > { # [doc = " Creates a new `TypedArena`."] fn default () -> TypedArena < T > { TypedArena { ptr : Cell :: new (ptr :: null_mut ()) , end : Cell :: new (ptr :: null_mut ()) , chunks : Default :: default () , _own : PhantomData , } } }
};
}
