// Generated macro for impl_29 (impl)
macro_rules! Depcrate_matches_patternimpl_29 {
() => {
// Module: crate::matches_pattern
// Provides: {"impl_29"}
// Dependencies: {}
impl Parse for FieldOrMethodPattern { fn parse (input : ParseStream) -> syn :: Result < Self > { let field_or_method : FieldOrMethod = input . parse () ? ; let underscore = input . parse :: < Option < Token ! [_] > > () ? ; match underscore { Some (underscore) if matches ! (field_or_method , FieldOrMethod :: Method (_)) => compile_err (underscore . spans [0] , "Don't match a method call against `_`. Just omit it instead." ,) , Some (_) => Ok (FieldOrMethodPattern { field_or_method , ref_token : None , matcher : None }) , None => Ok (FieldOrMethodPattern { field_or_method , ref_token : input . parse () ? , matcher : Some (input . parse () ?) , }) , } } }
};
}
