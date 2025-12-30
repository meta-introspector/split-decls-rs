// Generated macro for impl_196 (impl)
macro_rules! Depcrate_errorsimpl_196 {
() => {
// Module: crate::errors
// Provides: {"impl_196"}
// Dependencies: {}
impl Error { pub (crate) fn missed_end (name : QName , decoder : Decoder) -> Self { match decoder . decode (name . as_ref ()) { Ok (name) => IllFormedError :: MissingEndTag (name . into ()) . into () , Err (err) => err . into () , } } }
};
}
