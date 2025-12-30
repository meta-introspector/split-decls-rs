// Generated macro for impl_51 (impl)
macro_rules! Depcrate_flat_enumimpl_51 {
() => {
// Module: crate::flat_enum
// Provides: {"impl_51"}
// Dependencies: {}
impl Queue { fn push_back (& mut self , variant : NestedVariant) -> Result { # [cfg (feature = "alloc")] { self . inner . push_back (variant) ; Ok (()) } # [cfg (not (feature = "alloc"))] { let _ = variant ; Err (Error :: no_alloc ("nested enum variant")) } } fn pop_front (& mut self) -> Option < NestedVariant > { # [cfg (feature = "alloc")] { self . inner . pop_front () } # [cfg (not (feature = "alloc"))] { None } } fn is_empty (& self) -> bool { # [cfg (feature = "alloc")] { self . inner . is_empty () } # [cfg (not (feature = "alloc"))] { true } } }
};
}
