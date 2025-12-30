// Generated macro for impl_65 (impl)
macro_rules! Depcrate_blinkimpl_65 {
() => {
// Module: crate::blink
// Provides: {"impl_65"}
// Dependencies: {}
impl < A > Blink < A > { # [doc = " Creates new blink instance with provided allocator instance."] # [inline (always)] pub const fn new_in (alloc : A) -> Self { Blink { drop_list : DropList :: new () , alloc , } } # [doc = " Returns reference to allocator instance."] # [inline (always)] pub fn allocator (& self) -> & A { & self . alloc } # [doc = " Drops all allocated values."] # [doc = ""] # [doc = " Prefer to use `reset` method if associated allocator instance supports it."] # [inline (always)] pub fn drop_all (& mut self) { self . drop_list . reset () ; } }
};
}
