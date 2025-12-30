// Generated macro for character (function)
macro_rules! Depcrate_jsoncharacter {
() => {
// Module: crate::json
// Provides: {"character"}
// Dependencies: {}
fn character (input : & str) -> IResult < & str , char > { let (input , c) = none_of ("\"") (input) ? ; if c == '\\' { alt ((map_res (anychar , | c | { Ok (match c { '"' | '\\' | '/' => c , 'b' => '\x08' , 'f' => '\x0C' , 'n' => '\n' , 'r' => '\r' , 't' => '\t' , _ => return Err (()) , }) }) , preceded (char ('u') , unicode_escape) ,)) . parse (input) } else { Ok ((input , c)) } }
};
}
