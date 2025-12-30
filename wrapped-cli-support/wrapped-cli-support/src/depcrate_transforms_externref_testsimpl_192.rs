// Generated macro for impl_192 (impl)
macro_rules! Depcrate_transforms_externref_testsimpl_192 {
() => {
// Module: crate::transforms::externref::tests
// Provides: {"impl_192"}
// Dependencies: {}
impl < 'a > Parse < 'a > for Directive { fn parse (parser : Parser < 'a >) -> wast :: parser :: Result < Self > { use wast :: kw ; wast :: custom_keyword ! (externref_owned) ; wast :: custom_keyword ! (externref_borrowed) ; wast :: custom_keyword ! (other) ; let kind = if parser . peek :: < kw :: import > () ? { parser . parse :: < kw :: import > () ? ; DirectiveKind :: Import (parser . parse () ? , parser . parse () ?) } else { parser . parse :: < kw :: export > () ? ; DirectiveKind :: Export (parser . parse () ?) } ; let mut args = Vec :: new () ; parser . parens (| p | { let mut i = 0 ; while ! p . is_empty () { if parser . peek :: < externref_owned > () ? { parser . parse :: < externref_owned > () ? ; args . push ((i , true)) ; } else if parser . peek :: < externref_borrowed > () ? { parser . parse :: < externref_borrowed > () ? ; args . push ((i , false)) ; } else { parser . parse :: < other > () ? ; } i += 1 ; } Ok (()) }) ? ; let ret_externref = parser . parse :: < Option < externref_owned > > () ? . is_some () ; Ok (Directive { args , ret_externref , kind , }) } }
};
}
