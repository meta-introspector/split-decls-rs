// Generated macro for lookup_path_str (function)
macro_rules! Depcrate_pathslookup_path_str {
() => {
// Module: crate::paths
// Provides: {"lookup_path_str"}
// Dependencies: {}
# [doc = " Equivalent to a [`lookup_path`] after splitting the input string on `::`"] # [doc = ""] # [doc = " This function is expensive and should be used sparingly."] pub fn lookup_path_str (tcx : TyCtxt < '_ > , ns : PathNS , path : & str) -> Vec < DefId > { let path : Vec < Symbol > = path . split ("::") . map (Symbol :: intern) . collect () ; lookup_path (tcx , ns , & path) }
};
}
