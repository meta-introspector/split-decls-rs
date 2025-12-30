// Generated macro for generics_once (function)
macro_rules! Depcrate_errorgenerics_once {
() => {
// Module: crate::error
// Provides: {"generics_once"}
// Dependencies: {}
fn generics_once < 'a > (test : & 'a ItemFn , info : & FixtureInfo) -> Errors < 'a > { match (has_some_generics (test) , info . arguments . get_once ()) { (true , Some (once)) => Box :: new (std :: iter :: once (syn :: Error :: new_spanned (once , "Cannot apply #[once] on generic fixture." ,))) , _ => Box :: new (std :: iter :: empty ()) , } }
};
}
