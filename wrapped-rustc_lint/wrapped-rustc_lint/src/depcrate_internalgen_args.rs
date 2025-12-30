// Generated macro for gen_args (function)
macro_rules! Depcrate_internalgen_args {
() => {
// Module: crate::internal
// Provides: {"gen_args"}
// Dependencies: {}
fn gen_args (segment : & hir :: PathSegment < '_ >) -> String { if let Some (args) = & segment . args { let lifetimes = args . args . iter () . filter_map (| arg | { if let hir :: GenericArg :: Lifetime (lt) = arg { Some (lt . ident . to_string ()) } else { None } }) . collect :: < Vec < _ > > () ; if ! lifetimes . is_empty () { return format ! ("<{}>" , lifetimes . join (", ")) ; } } String :: new () }
};
}
