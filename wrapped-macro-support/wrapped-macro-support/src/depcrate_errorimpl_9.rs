// Generated macro for impl_9 (impl)
macro_rules! Depcrate_errorimpl_9 {
() => {
// Module: crate::error
// Provides: {"impl_9"}
// Dependencies: {}
impl Diagnostic { # [doc = " Generate a `Diagnostic` from an informational message with no Span"] pub fn error < T : Into < String > > (text : T) -> Diagnostic { Diagnostic { inner : Repr :: Single { text : text . into () , span : None , } , } } # [doc = " Generate a `Diagnostic` from a Span and an informational message"] pub fn span_error < T : Into < String > > (span : Span , text : T) -> Diagnostic { Diagnostic { inner : Repr :: Single { text : text . into () , span : Some ((span , span)) , } , } } # [doc = " Generate a `Diagnostic` from the span of any tokenizable object and a message"] pub fn spanned_error < T : Into < String > > (node : & dyn ToTokens , text : T) -> Diagnostic { Diagnostic { inner : Repr :: Single { text : text . into () , span : extract_spans (node) , } , } } # [doc = " Attempt to generate a `Diagnostic` from a vector of other `Diagnostic` instances."] # [doc = " If the `Vec` is empty, returns `Ok(())`, otherwise returns the new `Diagnostic`"] pub fn from_vec (diagnostics : Vec < Diagnostic >) -> Result < () , Diagnostic > { if diagnostics . is_empty () { Ok (()) } else { Err (Diagnostic { inner : Repr :: Multi { diagnostics } , }) } } # [doc = " Immediately trigger a panic from this `Diagnostic`"] # [allow (unconditional_recursion)] pub fn panic (& self) -> ! { match & self . inner { Repr :: Single { text , .. } => panic ! ("{}" , text) , Repr :: SynError (error) => panic ! ("{}" , error) , Repr :: Multi { diagnostics } => diagnostics [0] . panic () , } } }
};
}
