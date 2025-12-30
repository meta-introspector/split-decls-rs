// Generated macro for dispatch (macro)
macro_rules! Depcrate_parser_choicedispatch {
() => {
// Module: crate::parser::choice
// Provides: {"dispatch"}
// Dependencies: {}
# [doc = " `dispatch!` allows a parser to be constructed depending on earlier input, without forcing each"] # [doc = " branch to have the same type of parser"] # [doc = ""] # [doc = " ```"] # [doc = " use combine::{dispatch, any, token, satisfy, EasyParser, Parser};"] # [doc = ""] # [doc = " let mut parser = any().then(|e| {"] # [doc = "     dispatch!(e;"] # [doc = "         'a' => token('a'),"] # [doc = "         'b' => satisfy(|b| b == 'b'),"] # [doc = "         t if t == 'c' => any(),"] # [doc = "         _ => token('d')"] # [doc = "     )"] # [doc = " });"] # [doc = " assert_eq!(parser.easy_parse(\"aa\"), Ok(('a', \"\")));"] # [doc = " assert_eq!(parser.easy_parse(\"cc\"), Ok(('c', \"\")));"] # [doc = " assert_eq!(parser.easy_parse(\"cd\"), Ok(('d', \"\")));"] # [doc = " assert!(parser.easy_parse(\"ab\").is_err());"] # [doc = " ```"] # [macro_export] macro_rules ! dispatch { ($ match_expr : expr ; $ ($ ($ pat : pat) |+ $ (if $ pred : expr) ? => $ expr : expr) ,+ $ (,) ?) => { { $ crate :: dispatch_parser_impl ! { Dispatch [A B C D E F G H I J K L M N O P Q R S T U V X Y Z] [] $ ($ expr ,) + } fn check_parser < Input , P > (p : P) -> P where P : $ crate :: Parser < Input >, Input : $ crate :: Stream { p } let e = $ match_expr ; let parser = $ crate :: dispatch_inner ! (e [A B C D E F G H I J K L M N O P Q R S T U V X Y Z] [] $ ($ ($ pat) |+ $ (if $ pred) ? => $ expr ,) *) ; parser } } }
};
}
