// Generated macro for NoBlockEncoding (struct)
macro_rules! Depcrate_traitsNoBlockEncoding {
() => {
// Module: crate::traits
// Provides: {"NoBlockEncoding"}
// Dependencies: {}
# [doc = " Particular [`ManualBlockEncoding`] that indicates no actual encoding should"] # [doc = " be set in the block's descriptor."] # [doc = ""] # [doc = " This is used in a bit of a hackish way in order to share more code between"] # [doc = " the encoded and non-encoded paths."] pub (crate) struct NoBlockEncoding < A , R > where A : EncodeArguments , R : EncodeReturn , { _a : PhantomData < A > , _r : PhantomData < R > , }
};
}
