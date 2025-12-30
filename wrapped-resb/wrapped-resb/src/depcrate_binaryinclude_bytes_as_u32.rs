// Generated macro for include_bytes_as_u32 (macro)
macro_rules! Depcrate_binaryinclude_bytes_as_u32 {
() => {
// Module: crate::binary
// Provides: {"include_bytes_as_u32"}
// Dependencies: {}
# [doc = " Includes bytes as a properly aligned and sized `&[u32]`. This is required for `resb` deserialization."] # [macro_export] macro_rules ! include_bytes_as_u32 { ($ path : literal) => { const { # [repr (align (4))] pub struct AlignedAs < Bytes : ? Sized > { pub bytes : Bytes , } const B : & [u8] = & AlignedAs { bytes : * include_bytes ! ($ path) , } . bytes ; unsafe { core :: slice :: from_raw_parts (B . as_ptr () as * const u32 , B . len () / core :: mem :: size_of ::< u32 > () ,) } } } ; }
};
}
