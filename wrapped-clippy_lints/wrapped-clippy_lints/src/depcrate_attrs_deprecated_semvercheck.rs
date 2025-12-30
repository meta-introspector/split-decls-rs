// Generated macro for check (function)
macro_rules! Depcrate_attrs_deprecated_semvercheck {
() => {
// Module: crate::attrs::deprecated_semver
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & EarlyContext < '_ > , span : Span , lit : & MetaItemLit) { if let LitKind :: Str (is , _) = lit . kind && (is == sym :: TBD || Version :: parse (is . as_str ()) . is_ok ()) { return ; } span_lint (cx , DEPRECATED_SEMVER , span , "the since field must contain a semver-compliant version" ,) ; }
};
}
