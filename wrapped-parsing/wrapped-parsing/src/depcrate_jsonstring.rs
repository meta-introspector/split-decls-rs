// Generated macro for string (function)
macro_rules! Depcrate_jsonstring {
() => {
// Module: crate::json
// Provides: {"string"}
// Dependencies: {}
fn string (input : & str) -> IResult < & str , String > { delimited (char ('"') , fold_many0 (character , String :: new , | mut string , c | { string . push (c) ; string }) , char ('"') ,) . parse (input) }
};
}
