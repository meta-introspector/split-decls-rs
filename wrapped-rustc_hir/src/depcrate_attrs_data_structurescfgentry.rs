// Generated macro for CfgEntry (enum)
macro_rules! Depcrate_attrs_data_structuresCfgEntry {
() => {
// Module: crate::attrs::data_structures
// Provides: {"CfgEntry"}
// Dependencies: {}
# [derive (Encodable , Decodable , Clone , Debug , PartialEq , Eq , Hash)] # [derive (HashStable_Generic , PrintAttribute)] pub enum CfgEntry { All (ThinVec < CfgEntry > , Span) , Any (ThinVec < CfgEntry > , Span) , Not (Box < CfgEntry > , Span) , Bool (bool , Span) , NameValue { name : Symbol , name_span : Span , value : Option < (Symbol , Span) > , span : Span } , Version (Option < RustcVersion > , Span) , }
};
}
