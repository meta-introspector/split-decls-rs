// Generated macro for impl_900 (impl)
macro_rules! Depcrate_stream_zip_arrayimpl_900 {
() => {
// Module: crate::stream::zip::array
// Provides: {"impl_900"}
// Dependencies: {}
impl < S , const N : usize > ZipTrait for [S ; N] where S : IntoStream , { type Item = < Zip < S :: IntoStream , N > as Stream > :: Item ; type Stream = Zip < S :: IntoStream , N > ; fn zip (self) -> Self :: Stream { Zip :: new (self . map (| i | i . into_stream ())) } }
};
}
