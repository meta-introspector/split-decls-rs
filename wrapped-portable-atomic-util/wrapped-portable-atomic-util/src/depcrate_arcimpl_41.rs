// Generated macro for impl_41 (impl)
macro_rules! Depcrate_arcimpl_41 {
() => {
// Module: crate::arc
// Provides: {"impl_41"}
// Dependencies: {}
impl < T : Clone > Arc < T > { # [doc = " If we have the only reference to `T` then unwrap it. Otherwise, clone `T` and return the"] # [doc = " clone."] # [doc = ""] # [doc = " Assuming `arc_t` is of type `Arc<T>`, this function is functionally equivalent to"] # [doc = " `(*arc_t).clone()`, but will avoid cloning the inner value where possible."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use portable_atomic_util::Arc;"] # [doc = " use std::ptr;"] # [doc = ""] # [doc = " let inner = String::from(\"test\");"] # [doc = " let ptr = inner.as_ptr();"] # [doc = ""] # [doc = " let arc = Arc::new(inner);"] # [doc = " let inner = Arc::unwrap_or_clone(arc);"] # [doc = " // The inner value was not cloned"] # [doc = " assert!(ptr::eq(ptr, inner.as_ptr()));"] # [doc = ""] # [doc = " let arc = Arc::new(inner);"] # [doc = " let arc2 = arc.clone();"] # [doc = " let inner = Arc::unwrap_or_clone(arc);"] # [doc = " // Because there were 2 references, we had to clone the inner value."] # [doc = " assert!(!ptr::eq(ptr, inner.as_ptr()));"] # [doc = " // `arc2` is the last reference, so when we unwrap it we get back"] # [doc = " // the original `String`."] # [doc = " let inner = Arc::unwrap_or_clone(arc2);"] # [doc = " assert!(ptr::eq(ptr, inner.as_ptr()));"] # [doc = " ```"] # [inline] pub fn unwrap_or_clone (this : Self) -> T { Self :: try_unwrap (this) . unwrap_or_else (| arc | (* arc) . clone ()) } }
};
}
