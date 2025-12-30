// Generated macro for make_tag_pattern (function)
macro_rules! Depcrate_match_tokenmake_tag_pattern {
() => {
// Module: crate::match_token
// Provides: {"make_tag_pattern"}
// Dependencies: {}
fn make_tag_pattern (cx : & mut ExtCtxt , binding : Tokens , tag : Tag) -> Tokens { let kind = tag . kind . lift (cx) ; let mut fields = quote_tokens ! (& mut * cx , kind : $ kind ,) ; match tag . name { None => () , Some (name) => push_all (& mut fields , quote_tokens ! (& mut * cx , name : atom ! ($ name) ,)) , } quote_tokens ! (& mut * cx , :: tree_builder :: types :: TagToken ($ binding :: tokenizer :: Tag { $ fields .. })) }
};
}
