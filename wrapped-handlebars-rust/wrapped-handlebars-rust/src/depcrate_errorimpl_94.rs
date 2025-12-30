// Generated macro for impl_94 (impl)
macro_rules! Depcrate_errorimpl_94 {
() => {
// Module: crate::error
// Provides: {"impl_94"}
// Dependencies: {}
impl RenderError { # [deprecated (since = "5.0.0" , note = "Use RenderErrorReason instead")] pub fn new < T : AsRef < str > > (desc : T) -> RenderError { RenderErrorReason :: Other (desc . as_ref () . to_string ()) . into () } pub fn strict_error (path : Option < & String >) -> RenderError { RenderErrorReason :: MissingVariable (path . map (ToOwned :: to_owned)) . into () } # [deprecated (since = "5.0.0" , note = "Use RenderErrorReason::NestedError instead")] pub fn from_error < E > (_error_info : & str , cause : E) -> RenderError where E : StdError + Send + Sync + 'static , { RenderErrorReason :: NestedError (Box :: new (cause)) . into () } # [inline] pub (crate) fn is_unimplemented (& self) -> bool { matches ! (* self . reason , RenderErrorReason :: Unimplemented) } # [doc = " Get `RenderErrorReason` for this error"] pub fn reason (& self) -> & RenderErrorReason { self . reason . as_ref () } }
};
}
