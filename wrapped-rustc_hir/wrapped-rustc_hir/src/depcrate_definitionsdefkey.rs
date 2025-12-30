// Generated macro for DefKey (struct)
macro_rules! Depcrate_definitionsDefKey {
() => {
// Module: crate::definitions
// Provides: {"DefKey"}
// Dependencies: {}
# [doc = " A unique identifier that we can use to lookup a definition"] # [doc = " precisely. It combines the index of the definition's parent (if"] # [doc = " any) with a `DisambiguatedDefPathData`."] # [derive (Copy , Clone , PartialEq , Debug , Encodable , Decodable)] pub struct DefKey { # [doc = " The parent path."] pub parent : Option < DefIndex > , # [doc = " The identifier of this node."] pub disambiguated_data : DisambiguatedDefPathData , }
};
}
