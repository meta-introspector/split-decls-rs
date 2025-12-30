// Generated macro for ws (function)
macro_rules! Depcrate_jsonws {
() => {
// Module: crate::json
// Provides: {"ws"}
// Dependencies: {}
fn ws < 'a , O , E : ParseError < & 'a str > , F : Parser < & 'a str , O , E > > (f : F) -> impl Parser < & 'a str , O , E > { delimited (multispace0 , f , multispace0) }
};
}
