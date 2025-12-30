// Generated macro for path_ends_with (function)
macro_rules! Depcrate_referencespath_ends_with {
() => {
// Module: crate::references
// Provides: {"path_ends_with"}
// Dependencies: {}
# [doc = " Checks if a path ends with the given name reference."] # [doc = " Helper function for checking constructor usage patterns."] fn path_ends_with (path : Option < ast :: Path > , name_ref : & ast :: NameRef) -> bool { path . and_then (| path | path . segment ()) . and_then (| segment | segment . name_ref ()) . map_or (false , | segment | segment == * name_ref) }
};
}
