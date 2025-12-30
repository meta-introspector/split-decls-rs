// Generated macro for extend (function)
macro_rules! Depcrate_utilextend {
() => {
// Module: crate::util
// Provides: {"extend"}
// Dependencies: {}
# [inline] pub (crate) fn extend (base : & mut Vec < String > , slice : & [String]) { for i in slice { base . push (i . to_owned ()) ; } }
};
}
