// Generated macro for DrawingAreaErrorKind (enum)
macro_rules! Depcrate_drawing_areaDrawingAreaErrorKind {
() => {
// Module: crate::drawing::area
// Provides: {"DrawingAreaErrorKind"}
// Dependencies: {}
# [doc = " The error description of any drawing area API"] # [derive (Debug)] pub enum DrawingAreaErrorKind < E : Error + Send + Sync > { # [doc = " The error is due to drawing backend failure"] BackendError (DrawingErrorKind < E >) , # [doc = " We are not able to get the mutable reference of the backend,"] # [doc = " which indicates the drawing backend is current used by other"] # [doc = " drawing operation"] SharingError , # [doc = " The error caused by invalid layout"] LayoutError , }
};
}
