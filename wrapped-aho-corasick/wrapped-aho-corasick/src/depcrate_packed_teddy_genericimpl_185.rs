// Generated macro for impl_185 (impl)
macro_rules! Depcrate_packed_teddy_genericimpl_185 {
() => {
// Module: crate::packed::teddy::generic
// Provides: {"impl_185"}
// Dependencies: {}
impl < V : Vector , const BYTES : usize > Slim < V , BYTES > { # [doc = " Create a new \"slim\" Teddy searcher for the given patterns."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This panics when `BYTES` is any value other than 1, 2, 3 or 4."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers must ensure that this is okay to call in the current target for"] # [doc = " the current CPU."] # [inline (always)] pub (crate) unsafe fn new (patterns : Arc < Patterns >) -> Slim < V , BYTES > { assert ! (1 <= BYTES && BYTES <= 4 , "only 1, 2, 3 or 4 bytes are supported") ; let teddy = Teddy :: new (patterns) ; let masks = SlimMaskBuilder :: from_teddy (& teddy) ; Slim { teddy , masks } } # [doc = " Returns the approximate total amount of heap used by this type, in"] # [doc = " units of bytes."] # [inline (always)] pub (crate) fn memory_usage (& self) -> usize { self . teddy . memory_usage () } # [doc = " Returns the minimum length, in bytes, that a haystack must be in order"] # [doc = " to use it with this searcher."] # [inline (always)] pub (crate) fn minimum_len (& self) -> usize { V :: BYTES + (BYTES - 1) } }
};
}
