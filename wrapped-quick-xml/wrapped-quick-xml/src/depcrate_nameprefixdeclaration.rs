// Generated macro for PrefixDeclaration (enum)
macro_rules! Depcrate_namePrefixDeclaration {
() => {
// Module: crate::name
// Provides: {"PrefixDeclaration"}
// Dependencies: {}
# [doc = " A namespace prefix declaration, `xmlns` or `xmlns:<name>`, as defined in"] # [doc = " [XML Schema specification](https://www.w3.org/TR/xml-names11/#ns-decl)"] # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub enum PrefixDeclaration < 'a > { # [doc = " XML attribute binds a default namespace. Corresponds to `xmlns` in `xmlns=\"...\"`"] Default , # [doc = " XML attribute binds a specified prefix to a namespace. Corresponds to a"] # [doc = " `prefix` in `xmlns:prefix=\"...\"`, which is stored as payload of this variant."] Named (& 'a [u8]) , }
};
}
