// Generated macro for NamespaceEntry (enum)
macro_rules! Depcrate_jsNamespaceEntry {
() => {
// Module: crate::js
// Provides: {"NamespaceEntry"}
// Dependencies: {}
# [doc = " Namespaces can point to other namespaces or definitions"] enum NamespaceEntry { Namespace (BTreeMap < String , NamespaceEntry >) , Definition (String) , }
};
}
