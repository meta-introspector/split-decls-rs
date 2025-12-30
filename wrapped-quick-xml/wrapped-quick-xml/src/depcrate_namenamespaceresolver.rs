// Generated macro for NamespaceResolver (struct)
macro_rules! Depcrate_nameNamespaceResolver {
() => {
// Module: crate::name
// Provides: {"NamespaceResolver"}
// Dependencies: {}
# [doc = " A storage for currently defined namespace bindings, which is used to resolve"] # [doc = " prefixes into namespaces."] # [doc = ""] # [doc = " Holds all internal logic to push/pop namespaces with their levels."] # [derive (Debug , Clone)] pub struct NamespaceResolver { # [doc = " Buffer that contains names of namespace prefixes (the part between `xmlns:`"] # [doc = " and an `=`) and namespace values."] buffer : Vec < u8 > , # [doc = " A stack of namespace bindings to prefixes that currently in scope"] bindings : Vec < NamespaceBinding > , # [doc = " The number of open tags at the moment. We need to keep track of this to know which namespace"] # [doc = " declarations to remove when we encounter an `End` event."] nesting_level : u16 , }
};
}
