// Generated macro for SliceIter (struct)
macro_rules! Depcrate_pemSliceIter {
() => {
// Module: crate::pem
// Provides: {"SliceIter"}
// Dependencies: {}
# [doc = " Iterator over all PEM sections in a `&[u8]` slice."] pub struct SliceIter < 'a , T > { current : & 'a [u8] , _ty : PhantomData < T > , b64_buf : Vec < u8 > , }
};
}
