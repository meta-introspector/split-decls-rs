macro_rules! deps {
    () => {
        IdentUnraw!();
        FmtArguments!();
    };
}

macro_rules! fallback_explicit_named_args {
    () => {
        deps!();
        fn fallback_explicit_named_args (input : ParseStream) -> Result < FmtArguments > { let mut args = FmtArguments { named : BTreeSet :: new () , first_unnamed : None , } ; while ! input . is_empty () { if input . peek (Token ! [,]) && input . peek2 (Ident :: peek_any) && input . peek3 (Token ! [=]) && ! input . peek3 (Token ! [==]) { input . parse :: < Token ! [,] > () ? ; let ident : IdentUnraw = input . parse () ? ; input . parse :: < Token ! [=] > () ? ; args . named . insert (ident) ; } else { input . parse :: < TokenTree > () ? ; } } Ok (args) }
    };
}

fallback_explicit_named_args!();