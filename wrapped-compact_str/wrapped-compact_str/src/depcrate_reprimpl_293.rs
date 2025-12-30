// Generated macro for impl_293 (impl)
macro_rules! Depcrate_reprimpl_293 {
() => {
// Module: crate::repr
// Provides: {"impl_293"}
// Dependencies: {}
impl Clone for Repr { # [inline] fn clone (& self) -> Self { # [inline (never)] fn clone_heap (this : & Repr) -> Repr { Repr :: new (this . as_str ()) . unwrap_with_msg () } if self . is_heap_allocated () { clone_heap (self) } else { unsafe { core :: ptr :: read (self) } } } # [inline] fn clone_from (& mut self , source : & Self) { # [inline (never)] fn clone_from_heap (this : & mut Repr , source : & Repr) { unsafe { this . set_len (0) } ; this . push_str (source . as_str ()) ; } if source . is_heap_allocated () { clone_from_heap (self , source) } else { * self = unsafe { core :: ptr :: read (source) } } } }
};
}
