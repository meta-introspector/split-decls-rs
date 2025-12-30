// Generated macro for impl_54 (impl)
macro_rules! Depcrateimpl_54 {
() => {
// Module: crate
// Provides: {"impl_54"}
// Dependencies: {}
impl < T > Index < T > { # [doc = " Convenience function for creating new index."] # [must_use] pub (self) fn new (index : NonMaxUsize , generation : u64) -> Index < T > { Index { generation , index , phantom : PhantomData , } } # [doc = " Get the index as usize"] # [inline] pub (self) fn index (& self) -> usize { self . index . get () } }
};
}
