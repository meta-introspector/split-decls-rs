// Generated macro for convert_rule (function)
macro_rules! Depcrate_parserconvert_rule {
() => {
// Module: crate::parser
// Provides: {"convert_rule"}
// Dependencies: {}
fn convert_rule (rule : ParserRule < '_ >) -> AstRule { let ParserRule { name , ty , node , .. } = rule ; let expr = convert_node (node) ; AstRule { name , ty , expr } }
};
}
