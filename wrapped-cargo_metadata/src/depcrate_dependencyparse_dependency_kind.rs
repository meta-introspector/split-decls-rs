// Generated macro for parse_dependency_kind (function)
macro_rules! Depcrate_dependencyparse_dependency_kind {
() => {
// Module: crate::dependency
// Provides: {"parse_dependency_kind"}
// Dependencies: {}
# [doc = " The `kind` can be `null`, which is interpreted as the default - `Normal`."] pub (super) fn parse_dependency_kind < 'de , D > (d : D) -> Result < DependencyKind , D :: Error > where D : Deserializer < 'de > , { Deserialize :: deserialize (d) . map (| x : Option < _ > | x . unwrap_or_default ()) }
};
}
