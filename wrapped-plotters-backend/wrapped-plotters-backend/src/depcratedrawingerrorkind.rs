// Generated macro for DrawingErrorKind (enum)
macro_rules! DepcrateDrawingErrorKind {
() => {
// Module: crate
// Provides: {"DrawingErrorKind"}
// Dependencies: {}
# [doc = " The error produced by a drawing backend."] # [derive (Debug)] pub enum DrawingErrorKind < E : Error + Send + Sync > { # [doc = " A drawing backend error"] DrawingError (E) , # [doc = " A font rendering error"] FontError (Box < dyn Error + Send + Sync + 'static >) , }
};
}
