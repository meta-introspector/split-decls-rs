// Generated macro for minimal_quote_tt (macro)
macro_rules! Depcrate_quoteminimal_quote_tt {
() => {
// Module: crate::quote
// Provides: {"minimal_quote_tt"}
// Dependencies: {}
macro_rules ! minimal_quote_tt { (($ ($ t : tt) *)) => { Group :: new (Delimiter :: Parenthesis , minimal_quote ! ($ ($ t) *)) } ; ([$ ($ t : tt) *]) => { Group :: new (Delimiter :: Bracket , minimal_quote ! ($ ($ t) *)) } ; ({ $ ($ t : tt) * }) => { Group :: new (Delimiter :: Brace , minimal_quote ! ($ ($ t) *)) } ; (,) => { Punct :: new (',' , Spacing :: Alone) } ; (.) => { Punct :: new ('.' , Spacing :: Alone) } ; (;) => { Punct :: new (';' , Spacing :: Alone) } ; (!) => { Punct :: new ('!' , Spacing :: Alone) } ; (<) => { Punct :: new ('<' , Spacing :: Alone) } ; (>) => { Punct :: new ('>' , Spacing :: Alone) } ; (&) => { Punct :: new ('&' , Spacing :: Alone) } ; (=) => { Punct :: new ('=' , Spacing :: Alone) } ; (#) => { Punct :: new ('#' , Spacing :: Alone) } ; (|) => { Punct :: new ('|' , Spacing :: Alone) } ; (:) => { Punct :: new (':' , Spacing :: Alone) } ; (*) => { Punct :: new ('*' , Spacing :: Alone) } ; (_) => { Ident :: new ("_" , Span :: def_site ()) } ; ($ i : ident) => { Ident :: new (stringify ! ($ i) , Span :: def_site ()) } ; ($ lit : literal) => { stringify ! ($ lit) . parse ::< Literal > () . unwrap () } ; }
};
}
