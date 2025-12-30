// Generated macro for impl_408 (impl)
macro_rules! Depcrate_hirimpl_408 {
() => {
// Module: crate::hir
// Provides: {"impl_408"}
// Dependencies: {}
impl Safety { pub fn prefix_str (self) -> & 'static str { match self { Self :: Unsafe => "unsafe " , Self :: Safe => "" , } } # [inline] pub fn is_unsafe (self) -> bool { ! self . is_safe () } # [inline] pub fn is_safe (self) -> bool { match self { Self :: Unsafe => false , Self :: Safe => true , } } }
};
}
