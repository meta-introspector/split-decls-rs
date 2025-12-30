// Generated macro for Slice (struct)
macro_rules! Depcrate_ser_flavorsSlice {
() => {
// Module: crate::ser::flavors
// Provides: {"Slice"}
// Dependencies: {}
# [doc = " The `Slice` flavor is a storage flavor, storing the serialized (or otherwise modified) bytes into a plain"] # [doc = " `[u8]` slice. The `Slice` flavor resolves into a sub-slice of the original slice buffer."] pub struct Slice < 'a > { start : * mut u8 , cursor : * mut u8 , end : * mut u8 , _pl : PhantomData < & 'a [u8] > , }
};
}
