// Generated macro for create_pointer (macro)
macro_rules! Depcrate_ptrcreate_pointer {
() => {
// Module: crate::ptr
// Provides: {"create_pointer"}
// Dependencies: {}
macro_rules ! create_pointer { ($ ty : ty , $ free : path) => { impl Pointer for * mut $ ty { type T = $ ty ; # [inline] fn free (& mut self) { unsafe { let ptr = * self ; $ free (ptr . cast ()) ; } } # [inline] fn as_const_ptr (& self) -> * const Self :: T { self . cast () } # [inline] fn as_mut_ptr (& mut self) -> * mut Self :: T { * self } } } ; }
};
}
