// Generated macro for error_message (function)
macro_rules! Depcrate_validation_rules_possible_fragment_spreadserror_message {
() => {
// Module: crate::validation::rules::possible_fragment_spreads
// Provides: {"error_message"}
// Dependencies: {}
fn error_message (frag_name : Option < & str > , parent_type_name : & str , frag_type : & str) -> String { if let Some (frag_name) = frag_name { format ! ("Fragment \"{frag_name}\" cannot be spread here as objects of type \
             \"{parent_type_name}\" can never be of type \"{frag_type}\"" ,) } else { format ! ("Fragment cannot be spread here as objects of type \
             \"{parent_type_name}\" can never be of type \"{frag_type}\"" ,) } }
};
}
