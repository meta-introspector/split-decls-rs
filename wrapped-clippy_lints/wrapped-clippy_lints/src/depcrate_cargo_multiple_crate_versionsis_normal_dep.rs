// Generated macro for is_normal_dep (function)
macro_rules! Depcrate_cargo_multiple_crate_versionsis_normal_dep {
() => {
// Module: crate::cargo::multiple_crate_versions
// Provides: {"is_normal_dep"}
// Dependencies: {}
fn is_normal_dep (nodes : & [Node] , local_id : & PackageId , dep_id : & PackageId) -> bool { fn depends_on (node : & Node , dep_id : & PackageId) -> bool { node . deps . iter () . any (| dep | { dep . pkg == * dep_id && dep . dep_kinds . iter () . any (| info | matches ! (info . kind , DependencyKind :: Normal)) }) } nodes . iter () . filter (| node | depends_on (node , dep_id)) . any (| node | node . id == * local_id || is_normal_dep (nodes , local_id , & node . id)) }
};
}
