// Generated macro for with_index_to_label (function)
macro_rules! Depcrate_flattenerwith_index_to_label {
() => {
// Module: crate::flattener
// Provides: {"with_index_to_label"}
// Dependencies: {}
# [inline] fn with_index_to_label (index : & Index , label : Option < & Label > , f : impl FnOnce (& Label) -> sval :: Result ,) -> sval :: Result { if let Some (label) = label { return f (label) ; } LabelBuf :: from_index (index) ? . with_label (f) }
};
}
