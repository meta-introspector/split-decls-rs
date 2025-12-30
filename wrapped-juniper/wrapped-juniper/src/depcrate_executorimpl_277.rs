// Generated macro for impl_277 (impl)
macro_rules! Depcrate_executorimpl_277 {
() => {
// Module: crate::executor
// Provides: {"impl_277"}
// Dependencies: {}
impl < S > ExecutionError < S > { # [doc (hidden)] pub fn new (location : SourcePosition , path : & [& str] , error : FieldError < S >) -> ExecutionError < S > { ExecutionError { location , path : path . iter () . map (| s | (* s) . into ()) . collect () , error , } } # [doc = " The error message"] pub fn error (& self) -> & FieldError < S > { & self . error } # [doc = " The source location _in the query_ of the field that failed to resolve"] pub fn location (& self) -> & SourcePosition { & self . location } # [doc = " The path of fields leading to the field that generated this error"] pub fn path (& self) -> & [String] { & self . path } }
};
}
