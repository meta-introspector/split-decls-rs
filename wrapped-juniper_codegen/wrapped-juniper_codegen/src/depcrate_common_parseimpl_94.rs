// Generated macro for impl_94 (impl)
macro_rules! Depcrate_common_parseimpl_94 {
() => {
// Module: crate::common::parse
// Provides: {"impl_94"}
// Dependencies: {}
impl ParseBufferExt for ParseBuffer < '_ > { fn try_parse < T : Default + Parse + Token > (& self) -> syn :: Result < Option < T > > { Ok (if self . is_next :: < T > () { Some (self . parse () ?) } else { None }) } fn is_next < T : Default + Token > (& self) -> bool { self . lookahead1 () . peek (| _ | T :: default ()) } fn parse_any_ident (& self) -> syn :: Result < syn :: Ident > { self . call (syn :: Ident :: parse_any) } fn parse_maybe_wrapped_and_punctuated < T , W , P > (& self) -> syn :: Result < Punctuated < T , P > > where T : Parse , W : Default + Token + 'static , P : Default + Parse + Token , { Ok (if self . is_next :: < W > () { let inner ; if TypeId :: of :: < W > () == TypeId :: of :: < token :: Bracket > () { let _ = syn :: bracketed ! (inner in self) ; } else if TypeId :: of :: < W > () == TypeId :: of :: < token :: Brace > () { let _ = syn :: braced ! (inner in self) ; } else if TypeId :: of :: < W > () == TypeId :: of :: < token :: Paren > () { let _ = syn :: parenthesized ! (inner in self) ; } else { unimplemented ! ("ParseBufferExt::parse_maybe_wrapped_and_punctuated supports only brackets, \
                     braces and parentheses as wrappers." ,) ; } Punctuated :: parse_terminated (& inner) ? } else { Punctuated :: from_iter (iter :: once (self . parse :: < T > () ?)) }) } }
};
}
