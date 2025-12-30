// Generated macro for validate_repetition (function)
macro_rules! Depcrate_validatorvalidate_repetition {
() => {
// Module: crate::validator
// Provides: {"validate_repetition"}
// Dependencies: {}
fn validate_repetition < 'a , 'i : 'a > (rules : & 'a [ParserRule < 'i >]) -> Vec < Error < Rule > > { let mut result = vec ! [] ; let map = to_hash_map (rules) ; for rule in rules { let mut errors = rule . node . clone () . filter_map_top_down (| node | match node . expr { ParserExpr :: Rep (ref other) | ParserExpr :: RepOnce (ref other) | ParserExpr :: RepMin (ref other , _) => { if is_non_failing (& other . expr , & map , & mut vec ! []) { Some (Error :: new_from_span (ErrorVariant :: CustomError { message : "expression inside repetition cannot fail and will repeat \
                                     infinitely" . to_owned () } , node . span)) } else if is_non_progressing (& other . expr , & map , & mut vec ! []) { Some (Error :: new_from_span (ErrorVariant :: CustomError { message : "expression inside repetition is non-progressing and will repeat \
                                     infinitely" . to_owned () , } , node . span)) } else { None } } _ => None }) ; result . append (& mut errors) ; } result }
};
}
