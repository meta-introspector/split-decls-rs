// Generated macro for check_fn_sig (function)
macro_rules! Depcratecheck_fn_sig {
() => {
// Module: crate
// Provides: {"check_fn_sig"}
// Dependencies: {}
fn check_fn_sig (sig : & syn :: Signature) -> Result < () , () > { if sig . constness . is_none () && sig . asyncness . is_none () && sig . unsafety . is_none () && sig . abi . is_none () && sig . generics . params . is_empty () && sig . generics . where_clause . is_none () && sig . variadic . is_none () { Ok (()) } else { Err (()) } }
};
}
