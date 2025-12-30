// Generated macro for r#try_recurse (macro)
macro_rules! Depcrate_astr#try_recurse {
() => {
// Module: crate::ast
// Provides: {"r#try_recurse"}
// Dependencies: {}
macro_rules ! r#try_recurse { ($ expr : expr $ (,) ?) => { match $ expr { Result :: Err (error :: Error :: TooMuchRecursion) => { return Result :: Err (error :: Error :: TooMuchRecursion) ; } val => val , } } ; }
};
}
