// Generated macro for with_failure_message (function)
macro_rules! Depcrate_parse_utilwith_failure_message {
() => {
// Module: crate::parse::util
// Provides: {"with_failure_message"}
// Dependencies: {}
# [doc = " Transforms an error into a failure, while adding a message in the error detail."] pub fn with_failure_message < 'a , P , V > (mut parser : P , message : & 'a str) -> impl Parser < 'a , V > where P : Parser < 'a , V > , { move | input : Input < 'a > | parser (input) . map_err (| nom_err : Err < Error > | match nom_err { Err :: Error (e) => { Err :: Failure (e . with_detail (ErrorDetail :: new (input , message))) } e => e , }) }
};
}
