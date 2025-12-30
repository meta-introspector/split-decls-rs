// Generated macro for manual (function)
macro_rules! Depcrate_configmanual {
() => {
// Module: crate::config
// Provides: {"manual"}
// Dependencies: {}
# [cfg (test)] fn manual (fields : & [SchemaField]) -> String { fields . iter () . fold (String :: new () , | mut acc , (field , _ty , doc , default) | { let id = field . replace ('_' , ".") ; let name = format ! ("rust-analyzer.{id}") ; let doc = doc_comment_to_string (doc) ; if default . contains ('\n') { format_to_acc ! (acc , "## {name} {{#{id}}}\n\nDefault:\n```json\n{default}\n```\n\n{doc}\n\n") } else { format_to_acc ! (acc , "## {name} {{#{id}}}\n\nDefault: `{default}`\n\n{doc}\n\n") } }) }
};
}
