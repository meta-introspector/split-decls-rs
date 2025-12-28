macro_rules! deps {
    () => {
        Transparent!();
        Display!();
        Attrs!();
        Fmt!();
    };
}

macro_rules! parse_error_attribute {
    () => {
        deps!();
        fn parse_error_attribute < 'a > (attrs : & mut Attrs < 'a > , attr : & 'a Attribute) -> Result < () > { mod kw { syn :: custom_keyword ! (transparent) ; syn :: custom_keyword ! (fmt) ; } attr . parse_args_with (| input : ParseStream | { let lookahead = input . lookahead1 () ; let fmt = if lookahead . peek (LitStr) { input . parse :: < LitStr > () ? } else if lookahead . peek (kw :: transparent) { let kw : kw :: transparent = input . parse () ? ; if attrs . transparent . is_some () { return Err (Error :: new_spanned (attr , "duplicate #[error(transparent)] attribute" ,)) ; } attrs . transparent = Some (Transparent { original : attr , span : kw . span , }) ; return Ok (()) ; } else if lookahead . peek (kw :: fmt) { input . parse :: < kw :: fmt > () ? ; input . parse :: < Token ! [=] > () ? ; let path : ExprPath = input . parse () ? ; if attrs . fmt . is_some () { return Err (Error :: new_spanned (attr , "duplicate #[error(fmt = ...)] attribute" ,)) ; } attrs . fmt = Some (Fmt { original : attr , path , }) ; return Ok (()) ; } else { return Err (lookahead . error ()) ; } ; let args = if input . is_empty () || input . peek (Token ! [,]) && input . peek2 (End) { input . parse :: < Option < Token ! [,] > > () ? ; TokenStream :: new () } else { parse_token_expr (input , false) ? } ; let requires_fmt_machinery = ! args . is_empty () ; let display = Display { original : attr , fmt , args , requires_fmt_machinery , has_bonus_display : false , infinite_recursive : false , implied_bounds : Set :: new () , bindings : Vec :: new () , } ; if attrs . display . is_some () { return Err (Error :: new_spanned (attr , "only one #[error(...)] attribute is allowed" ,)) ; } attrs . display = Some (display) ; Ok (()) }) }
    };
}

parse_error_attribute!();