// Generated macro for impl_729 (impl)
macro_rules! Depcrate_parser_tokenimpl_729 {
() => {
// Module: crate::parser::token
// Provides: {"impl_729"}
// Dependencies: {}
impl < Input , C , E , T > Parser < Input > for Tokens < C , E , T , Input > where C : FnMut (T :: Item , Input :: Token) -> bool , E : for < 's > ErrorInfo < 's , Input :: Token , Input :: Range > , T : Clone + IntoIterator , Input : Stream , { type Output = T ; type PartialState = () ; # [inline] fn parse_lazy (& mut self , input : & mut Input) -> ParseResult < T , Input :: Error > { let start = input . position () ; let mut committed = false ; for c in self . tokens . clone () { match crate :: stream :: uncons (input) { CommitOk (other) | PeekOk (other) => { if ! (self . cmp) (c , other . clone ()) { return if committed { let mut errors = < Input as StreamOnce > :: Error :: from_error (start , StreamError :: unexpected_token (other) ,) ; errors . add_expected (& self . expected) ; CommitErr (errors) } else { PeekErr (< Input as StreamOnce > :: Error :: empty (start) . into ()) } ; } committed = true ; } PeekErr (mut error) => { error . error . set_position (start) ; return if committed { CommitErr (error . error) } else { PeekErr (error) } ; } CommitErr (mut error) => { error . set_position (start) ; return CommitErr (error) ; } } } if committed { CommitOk (self . tokens . clone ()) } else { PeekOk (self . tokens . clone ()) } } fn add_error (& mut self , errors : & mut Tracked < < Input as StreamOnce > :: Error >) { errors . error . add_expected (& self . expected) ; } }
};
}
