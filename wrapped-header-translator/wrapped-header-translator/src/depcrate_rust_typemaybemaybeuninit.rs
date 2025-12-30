// Generated macro for maybemaybeuninit (function)
macro_rules! Depcrate_rust_typemaybemaybeuninit {
() => {
// Module: crate::rust_type
// Provides: {"maybemaybeuninit"}
// Dependencies: {}
fn maybemaybeuninit (read : bool , inner : impl Display) -> impl Display { FormatterFn (move | f | { if read { write ! (f , "{inner}") } else { write ! (f , "MaybeUninit<{inner}>") } }) }
};
}
