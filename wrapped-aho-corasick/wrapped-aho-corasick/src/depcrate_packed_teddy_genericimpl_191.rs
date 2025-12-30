// Generated macro for impl_191 (impl)
macro_rules! Depcrate_packed_teddy_genericimpl_191 {
() => {
// Module: crate::packed::teddy::generic
// Provides: {"impl_191"}
// Dependencies: {}
impl < V : FatVector , const BYTES : usize > Fat < V , BYTES > { # [doc = " Create a new \"fat\" Teddy searcher for the given patterns."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This panics when `BYTES` is any value other than 1, 2, 3 or 4."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers must ensure that this is okay to call in the current target for"] # [doc = " the current CPU."] # [inline (always)] pub (crate) unsafe fn new (patterns : Arc < Patterns >) -> Fat < V , BYTES > { assert ! (1 <= BYTES && BYTES <= 4 , "only 1, 2, 3 or 4 bytes are supported") ; let teddy = Teddy :: new (patterns) ; let masks = FatMaskBuilder :: from_teddy (& teddy) ; Fat { teddy , masks } } # [doc = " Returns the approximate total amount of heap used by this type, in"] # [doc = " units of bytes."] # [inline (always)] pub (crate) fn memory_usage (& self) -> usize { self . teddy . memory_usage () } # [doc = " Returns the minimum length, in bytes, that a haystack must be in order"] # [doc = " to use it with this searcher."] # [inline (always)] pub (crate) fn minimum_len (& self) -> usize { V :: Half :: BYTES + (BYTES - 1) } }
};
}
