// Generated macro for impl_1267 (impl)
macro_rules! Depcrate_stackimpl_1267 {
() => {
// Module: crate::stack
// Provides: {"impl_1267"}
// Dependencies: {}
impl < T : Stackable > ForeignType for Stack < T > { type CType = T :: StackType ; type Ref = StackRef < T > ; # [inline] unsafe fn from_ptr (ptr : * mut T :: StackType) -> Stack < T > { assert ! (! ptr . is_null () , "Must not instantiate a Stack from a null-ptr - use Stack::new() in \
             that case") ; Stack (ptr) } # [inline] fn as_ptr (& self) -> * mut T :: StackType { self . 0 } }
};
}
