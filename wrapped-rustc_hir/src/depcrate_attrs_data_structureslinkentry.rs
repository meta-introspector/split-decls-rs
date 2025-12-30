// Generated macro for LinkEntry (struct)
macro_rules! Depcrate_attrs_data_structuresLinkEntry {
() => {
// Module: crate::attrs::data_structures
// Provides: {"LinkEntry"}
// Dependencies: {}
# [derive (Debug , Encodable , Decodable , Clone , HashStable_Generic , PrintAttribute)] pub struct LinkEntry { pub span : Span , pub kind : NativeLibKind , pub name : Symbol , pub cfg : Option < CfgEntry > , pub verbatim : Option < bool > , pub import_name_type : Option < (PeImportNameType , Span) > , }
};
}
