// Generated macro for tag_base_type (function)
macro_rules! Depcrate_debuginfotag_base_type {
() => {
// Module: crate::debuginfo
// Provides: {"tag_base_type"}
// Dependencies: {}
# [doc = " Extract the type with which we want to describe the tag of the given enum or coroutine."] pub fn tag_base_type < 'tcx > (tcx : TyCtxt < 'tcx > , enum_type_and_layout : TyAndLayout < 'tcx >) -> Ty < 'tcx > { tag_base_type_opt (tcx , enum_type_and_layout) . unwrap_or_else (| | { bug ! ("tag_base_type() called for enum without tag: {:?}" , enum_type_and_layout) }) }
};
}
