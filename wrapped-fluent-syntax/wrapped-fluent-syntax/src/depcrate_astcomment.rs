// Generated macro for Comment (struct)
macro_rules! Depcrate_astComment {
() => {
// Module: crate::ast
// Provides: {"Comment"}
// Dependencies: {}
# [doc = " Fluent [`Comment`]."] # [doc = ""] # [doc = " In Fluent, comments may be standalone, or associated with"] # [doc = " an entry such as [`Term`] or [`Message`]."] # [doc = ""] # [doc = " When used as a standalone [`Entry`], comments may appear in one of"] # [doc = " three levels:"] # [doc = ""] # [doc = " * Standalone comment"] # [doc = " * Group comment associated with a group of messages"] # [doc = " * Resource comment associated with the whole resource"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use fluent_syntax::parser;"] # [doc = " use fluent_syntax::ast;"] # [doc = ""] # [doc = " let ftl = r#\""] # [doc = " ## A standalone level comment"] # [doc = " \"#;"] # [doc = ""] # [doc = " let resource = parser::parse(ftl)"] # [doc = "     .expect(\"Failed to parse an FTL resource.\");"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     resource,"] # [doc = "     ast::Resource {"] # [doc = "         body: vec!["] # [doc = "             ast::Entry::Comment(ast::Comment {"] # [doc = "                 content: vec!["] # [doc = "                     \"A standalone level comment\""] # [doc = "                 ]"] # [doc = "             })"] # [doc = "         ]"] # [doc = "     }"] # [doc = " );"] # [doc = " ```"] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] # [cfg_attr (feature = "serde" , serde (from = "helper::CommentDef<S>"))] pub struct Comment < S > { pub content : Vec < S > , }
};
}
