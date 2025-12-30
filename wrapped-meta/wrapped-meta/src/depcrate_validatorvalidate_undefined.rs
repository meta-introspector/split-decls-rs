// Generated macro for validate_undefined (function)
macro_rules! Depcrate_validatorvalidate_undefined {
() => {
// Module: crate::validator
// Provides: {"validate_undefined"}
// Dependencies: {}
# [doc = " Validates that the given `definitions` do not contain any undefined rules."] # [allow (clippy :: ptr_arg)] pub fn validate_undefined < 'i > (definitions : & Vec < Span < 'i > > , called_rules : & Vec < Span < 'i > > ,) -> Vec < Error < Rule > > { let mut errors = vec ! [] ; let definitions : HashSet < _ > = definitions . iter () . map (| span | span . as_str ()) . collect () ; for rule in called_rules { let name = rule . as_str () ; if ! definitions . contains (name) && ! BUILTINS . contains (name) { errors . push (Error :: new_from_span (ErrorVariant :: CustomError { message : format ! ("rule {} is undefined" , name) , } , * rule ,)) } } errors }
};
}
