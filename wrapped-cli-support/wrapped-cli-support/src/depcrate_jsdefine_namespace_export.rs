// Generated macro for define_namespace_export (function)
macro_rules! Depcrate_jsdefine_namespace_export {
() => {
// Module: crate::js
// Provides: {"define_namespace_export"}
// Dependencies: {}
fn define_namespace_export (namespaces : & mut BTreeMap < String , NamespaceEntry > , ns_path : & [String] , name : & str , def : String ,) -> Result < () , Error > { let ns = match namespaces . entry (ns_path [0] . to_string ()) . or_default () { NamespaceEntry :: Namespace (ns) => ns , NamespaceEntry :: Definition (_) => { bail ! ("Cannot define {name} as both a namespace and a definition") } } ; if ns_path . len () > 1 { define_namespace_export (ns , & ns_path [1 ..] , name , def) ? ; return Ok (()) ; } ; ns . insert (name . to_string () , NamespaceEntry :: Definition (def)) ; Ok (()) }
};
}
