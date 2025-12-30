// Generated macro for CsrError (enum)
macro_rules! Depcrate_csrCsrError {
() => {
// Module: crate::csr
// Provides: {"CsrError"}
// Dependencies: {}
# [doc = " The error type for fallible operations with `Csr`."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum CsrError { # [doc = " Both vertex indexes go outside the graph."] IndicesOutBounds (usize , usize) , }
};
}
