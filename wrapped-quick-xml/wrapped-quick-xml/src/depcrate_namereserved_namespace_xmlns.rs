// Generated macro for RESERVED_NAMESPACE_XMLNS (const)
macro_rules! Depcrate_nameRESERVED_NAMESPACE_XMLNS {
() => {
// Module: crate::name
// Provides: {"RESERVED_NAMESPACE_XMLNS"}
// Dependencies: {}
# [doc = " That constant define the one of [reserved namespaces] for the xml standard."] # [doc = ""] # [doc = " The prefix `xmlns` is used only to declare namespace bindings and is by definition bound"] # [doc = " to the namespace name `http://www.w3.org/2000/xmlns/`. It must not be declared or"] # [doc = " undeclared. Other prefixes must not be bound to this namespace name, and it must not be"] # [doc = " declared as the default namespace. Element names must not have the prefix `xmlns`."] # [doc = ""] # [doc = " [reserved namespaces]: https://www.w3.org/TR/xml-names11/#xmlReserved"] const RESERVED_NAMESPACE_XMLNS : (Prefix , Namespace) = (Prefix (b"xmlns") , Namespace (b"http://www.w3.org/2000/xmlns/") ,) ;
};
}
