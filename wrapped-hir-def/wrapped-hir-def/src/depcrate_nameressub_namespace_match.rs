// Generated macro for sub_namespace_match (function)
macro_rules! Depcrate_nameressub_namespace_match {
() => {
// Module: crate::nameres
// Provides: {"sub_namespace_match"}
// Dependencies: {}
# [doc = " Quoted from [rustc]:"] # [doc = " Macro namespace is separated into two sub-namespaces, one for bang macros and"] # [doc = " one for attribute-like macros (attributes, derives)."] # [doc = " We ignore resolutions from one sub-namespace when searching names in scope for another."] # [doc = ""] # [doc = " [rustc]: https://github.com/rust-lang/rust/blob/1.69.0/compiler/rustc_resolve/src/macros.rs#L75"] fn sub_namespace_match (candidate : Option < MacroSubNs > , expected : Option < MacroSubNs >) -> bool { match (candidate , expected) { (Some (candidate) , Some (expected)) => candidate == expected , _ => true , } }
};
}
