// Generated macro for extract_clippy_version_value (function)
macro_rules! Depcrate_lint_without_lint_passextract_clippy_version_value {
() => {
// Module: crate::lint_without_lint_pass
// Provides: {"extract_clippy_version_value"}
// Dependencies: {}
# [doc = " This function extracts the version value of a `clippy::version` attribute if the given value has"] # [doc = " one"] pub (super) fn extract_clippy_version_value (cx : & LateContext < '_ > , item : & '_ Item < '_ >) -> Option < Symbol > { let attrs = cx . tcx . hir_attrs (item . hir_id ()) ; attrs . iter () . find_map (| attr | { if let hir :: Attribute :: Unparsed (attr_kind) = & attr && let [tool_name , attr_name] = & attr_kind . path . segments [..] && tool_name . name == sym :: clippy && attr_name . name == sym :: version && let Some (version) = attr . value_str () { Some (version) } else { None } }) }
};
}
