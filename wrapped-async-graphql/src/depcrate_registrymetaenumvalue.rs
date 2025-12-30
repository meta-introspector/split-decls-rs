// Generated macro for MetaEnumValue (struct)
macro_rules! Depcrate_registryMetaEnumValue {
() => {
// Module: crate::registry
// Provides: {"MetaEnumValue"}
// Dependencies: {}
# [derive (Clone)] pub struct MetaEnumValue { pub name : String , pub description : Option < String > , pub deprecation : Deprecation , pub visible : Option < MetaVisibleFn > , pub inaccessible : bool , pub tags : Vec < String > , pub directive_invocations : Vec < MetaDirectiveInvocation > , }
};
}
