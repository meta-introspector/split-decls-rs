// Generated macro for validate_pest_keywords (function)
macro_rules! Depcrate_validatorvalidate_pest_keywords {
() => {
// Module: crate::validator
// Provides: {"validate_pest_keywords"}
// Dependencies: {}
# [doc = " Validates that the given `definitions` do not contain any Pest keywords."] # [allow (clippy :: ptr_arg)] pub fn validate_pest_keywords (definitions : & Vec < Span < '_ > >) -> Vec < Error < Rule > > { let mut errors = vec ! [] ; for definition in definitions { let name = definition . as_str () ; if PEST_KEYWORDS . contains (name) { errors . push (Error :: new_from_span (ErrorVariant :: CustomError { message : format ! ("{} is a pest keyword" , name) , } , * definition ,)) } } errors }
};
}
