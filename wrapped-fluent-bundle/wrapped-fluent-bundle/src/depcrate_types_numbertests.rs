// Generated macro for tests (module)
macro_rules! Depcrate_types_numbertests {
() => {
// Module: crate::types::number
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: types :: FluentValue ; # [test] fn value_from_copy_ref () { let x = 1i16 ; let y = & x ; let z : FluentValue = y . into () ; assert_eq ! (z , FluentValue :: try_number ("1")) ; } }
};
}
