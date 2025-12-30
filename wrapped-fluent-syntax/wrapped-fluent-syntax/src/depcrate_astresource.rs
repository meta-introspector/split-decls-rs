// Generated macro for Resource (struct)
macro_rules! Depcrate_astResource {
() => {
// Module: crate::ast
// Provides: {"Resource"}
// Dependencies: {}
# [doc = " Root node of a Fluent Translation List."] # [doc = ""] # [doc = " A [`Resource`] contains a body with a list of [`Entry`] nodes."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use fluent_syntax::parser;"] # [doc = " use fluent_syntax::ast;"] # [doc = ""] # [doc = " let ftl = \"\";"] # [doc = ""] # [doc = " let resource = parser::parse(ftl)"] # [doc = "     .expect(\"Failed to parse an FTL resource.\");"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     resource,"] # [doc = "     ast::Resource {"] # [doc = "         body: vec![]"] # [doc = "     }"] # [doc = " );"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Resource < S > { pub body : Vec < Entry < S > > , }
};
}
