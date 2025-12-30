// Generated macro for macro_187 (macro)
macro_rules! Depcrate_errormacro_187 {
() => {
// Module: crate::error
// Provides: {"macro_187"}
// Dependencies: {}
error ! (has_lifetimes , E0001 , "Cannot derive `Arbitrary` for types with generic lifetimes, such as: \
     `struct Foo<'a> { bar: &'a str }`. Currently, strategies for such types \
     are impossible to define.") ;
};
}
