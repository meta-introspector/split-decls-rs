// Generated macro for ReadIter (struct)
macro_rules! Depcrate_pemReadIter {
() => {
// Module: crate::pem
// Provides: {"ReadIter"}
// Dependencies: {}
# [doc = " Extract and return all PEM sections by reading `rd`."] # [cfg (feature = "std")] pub struct ReadIter < R , T > { rd : R , _ty : PhantomData < T > , line : Vec < u8 > , b64_buf : Vec < u8 > , }
};
}
