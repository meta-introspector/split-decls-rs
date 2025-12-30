// Generated macro for Crate (struct)
macro_rules! Depcrate_astCrate {
() => {
// Module: crate::ast
// Provides: {"Crate"}
// Dependencies: {}
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct Crate { # [doc = " Must be equal to `CRATE_NODE_ID` after the crate root is expanded, but may hold"] # [doc = " expansion placeholders or an unassigned value (`DUMMY_NODE_ID`) before that."] pub id : NodeId , pub attrs : AttrVec , pub items : ThinVec < Box < Item > > , pub spans : ModSpans , pub is_placeholder : bool , }
};
}
