// Generated macro for MultiExporter (struct)
macro_rules! Depcrate_exportMultiExporter {
() => {
// Module: crate::export
// Provides: {"MultiExporter"}
// Dependencies: {}
# [doc = " A `DataExporter` that forks to multiple `DataExporter`s."] # [derive (Default)] pub struct MultiExporter (Vec < Box < dyn DataExporter > >) ;
};
}
