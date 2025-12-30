// Generated macro for separate_with_comma_and (function)
macro_rules! Depcrate_rust_typeseparate_with_comma_and {
() => {
// Module: crate::rust_type
// Provides: {"separate_with_comma_and"}
// Dependencies: {}
fn separate_with_comma_and < T : Display > (items : impl IntoIterator < Item = T > + Clone) -> impl Display { FormatterFn (move | f | { let mut iter = items . clone () . into_iter () . peekable () ; if let Some (item) = iter . next () { write ! (f , "{item}") ? ; } while let Some (item) = iter . next () { if iter . peek () . is_some () { write ! (f , ", {item}") ? ; } else { write ! (f , " and {item}") ? ; } } Ok (()) }) }
};
}
