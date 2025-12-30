// Generated macro for VisibilityKind (enum)
macro_rules! Depcrate_astVisibilityKind {
() => {
// Module: crate::ast
// Provides: {"VisibilityKind"}
// Dependencies: {}
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum VisibilityKind { Public , Restricted { path : Box < Path > , id : NodeId , shorthand : bool } , Inherited , }
};
}
