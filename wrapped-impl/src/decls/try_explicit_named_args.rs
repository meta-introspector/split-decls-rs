macro_rules! deps {
    () => {
        IdentUnraw!();
        FmtArguments!();
    };
}

macro_rules! try_explicit_named_args {
    () => {
        deps!();
        fn try_explicit_named_args (input : ParseStream) -> Result < FmtArguments > { let mut syn_full = None ; let mut args = FmtArguments { named : BTreeSet :: new () , first_unnamed : None , } ; while ! input . is_empty () { input . parse :: < Token ! [,] > () ? ; if input . is_empty () { break ; } let mut begin_unnamed = None ; if input . peek (Ident :: peek_any) && input . peek2 (Token ! [=]) && ! input . peek2 (Token ! [==]) { let ident : IdentUnraw = input . parse () ? ; input . parse :: < Token ! [=] > () ? ; args . named . insert (ident) ; } else { begin_unnamed = Some (input . fork ()) ; } let ahead = input . fork () ; if * syn_full . get_or_insert_with (is_syn_full) && ahead . parse :: < Expr > () . is_ok () { input . advance_to (& ahead) ; } else { scan_expr (input) ? ; } if let Some (begin_unnamed) = begin_unnamed { if args . first_unnamed . is_none () { args . first_unnamed = Some (between (& begin_unnamed , input)) ; } } } Ok (args) }
    };
}

try_explicit_named_args!();