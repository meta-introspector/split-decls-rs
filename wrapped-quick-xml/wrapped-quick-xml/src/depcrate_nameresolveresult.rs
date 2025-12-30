// Generated macro for ResolveResult (enum)
macro_rules! Depcrate_nameResolveResult {
() => {
// Module: crate::name
// Provides: {"ResolveResult"}
// Dependencies: {}
# [doc = " Result of [prefix] resolution which creates by [`NamespaceResolver::resolve`],"] # [doc = " [`NsReader::read_resolved_event`] and"] # [doc = " [`NsReader::read_resolved_event_into`] methods."] # [doc = ""] # [doc = " [prefix]: Prefix"] # [doc = " [`NsReader::read_resolved_event`]: crate::reader::NsReader::read_resolved_event"] # [doc = " [`NsReader::read_resolved_event_into`]: crate::reader::NsReader::read_resolved_event_into"] # [derive (Clone , PartialEq , Eq , Hash)] pub enum ResolveResult < 'ns > { # [doc = " Qualified name does not contain prefix, and resolver does not define"] # [doc = " default namespace, so name is not bound to any namespace"] Unbound , # [doc = " [`Prefix`] resolved to the specified namespace"] Bound (Namespace < 'ns >) , # [doc = " Specified prefix was not found in scope"] Unknown (Vec < u8 >) , }
};
}
