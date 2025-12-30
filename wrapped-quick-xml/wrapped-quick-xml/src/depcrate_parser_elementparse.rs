// Generated macro for parse (function)
macro_rules! Depcrate_parser_elementparse {
() => {
// Module: crate::parser::element
// Provides: {"parse"}
// Dependencies: {}
# [test] fn parse () { use pretty_assertions :: assert_eq ; use ElementParser :: * ; # [doc = " Returns `Ok(pos)` with the position in the buffer where element is ended."] # [doc = ""] # [doc = " Returns `Err(internal_state)` if parsing does not done yet."] fn parse_element (bytes : & [u8] , mut parser : ElementParser) -> Result < usize , ElementParser > { match parser . feed (bytes) { Some (i) => Ok (i) , None => Err (parser) , } } assert_eq ! (parse_element (b"" , Outside) , Err (Outside)) ; assert_eq ! (parse_element (b"" , SingleQ) , Err (SingleQ)) ; assert_eq ! (parse_element (b"" , DoubleQ) , Err (DoubleQ)) ; assert_eq ! (parse_element (b"'" , Outside) , Err (SingleQ)) ; assert_eq ! (parse_element (b"'" , SingleQ) , Err (Outside)) ; assert_eq ! (parse_element (b"'" , DoubleQ) , Err (DoubleQ)) ; assert_eq ! (parse_element (b"\"" , Outside) , Err (DoubleQ)) ; assert_eq ! (parse_element (b"\"" , SingleQ) , Err (SingleQ)) ; assert_eq ! (parse_element (b"\"" , DoubleQ) , Err (Outside)) ; assert_eq ! (parse_element (b">" , Outside) , Ok (0)) ; assert_eq ! (parse_element (b">" , SingleQ) , Err (SingleQ)) ; assert_eq ! (parse_element (b">" , DoubleQ) , Err (DoubleQ)) ; assert_eq ! (parse_element (b"''>" , Outside) , Ok (2)) ; assert_eq ! (parse_element (b"''>" , SingleQ) , Err (SingleQ)) ; assert_eq ! (parse_element (b"''>" , DoubleQ) , Err (DoubleQ)) ; }
};
}
