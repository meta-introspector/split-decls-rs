// Generated macro for NamespaceError (enum)
macro_rules! Depcrate_nameNamespaceError {
() => {
// Module: crate::name
// Provides: {"NamespaceError"}
// Dependencies: {}
# [doc = " Some namespace was invalid"] # [derive (Debug , Clone , PartialEq , Eq)] pub enum NamespaceError { # [doc = " Specified namespace prefix is unknown, cannot resolve namespace for it"] UnknownPrefix (Vec < u8 >) , # [doc = " Attempts to bind the `xml` prefix to something other than `http://www.w3.org/XML/1998/namespace`."] # [doc = ""] # [doc = " `xml` prefix can be bound only to `http://www.w3.org/XML/1998/namespace`."] # [doc = ""] # [doc = " Contains the namespace to which `xml` tried to be bound."] InvalidXmlPrefixBind (Vec < u8 >) , # [doc = " Attempts to bind the `xmlns` prefix."] # [doc = ""] # [doc = " `xmlns` prefix is always bound to `http://www.w3.org/2000/xmlns/` and cannot be bound"] # [doc = " to any other namespace or even to `http://www.w3.org/2000/xmlns/`."] # [doc = ""] # [doc = " Contains the namespace to which `xmlns` tried to be bound."] InvalidXmlnsPrefixBind (Vec < u8 >) , # [doc = " Attempts to bind some prefix (except `xml`) to `http://www.w3.org/XML/1998/namespace`."] # [doc = ""] # [doc = " Only `xml` prefix can be bound to `http://www.w3.org/XML/1998/namespace`."] # [doc = ""] # [doc = " Contains the prefix that is tried to be bound."] InvalidPrefixForXml (Vec < u8 >) , # [doc = " Attempts to bind some prefix to `http://www.w3.org/2000/xmlns/`."] # [doc = ""] # [doc = " `http://www.w3.org/2000/xmlns/` cannot be bound to any prefix, even to `xmlns`."] # [doc = ""] # [doc = " Contains the prefix that is tried to be bound."] InvalidPrefixForXmlns (Vec < u8 >) , }
};
}
