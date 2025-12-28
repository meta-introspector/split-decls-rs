macro_rules! deps {
    () => {
        Parser!();
        TokenSet!();
    };
}

macro_rules! delimited {
    () => {
        deps!();
        # [doc = " The `parser` passed this is required to at least consume one token if it returns `true`."] # [doc = " If the `parser` returns false, parsing will stop."] fn delimited (p : & mut Parser < '_ > , bra : SyntaxKind , ket : SyntaxKind , delim : SyntaxKind , unexpected_delim_message : impl Fn () -> String , first_set : TokenSet , mut parser : impl FnMut (& mut Parser < '_ >) -> bool ,) { p . bump (bra) ; while ! p . at (ket) && ! p . at (EOF) { if p . at (delim) { let m = p . start () ; p . error (unexpected_delim_message ()) ; p . bump (delim) ; m . complete (p , ERROR) ; continue ; } if ! parser (p) { break ; } if ! p . eat (delim) { if p . at_ts (first_set) { p . error (format ! ("expected {delim:?}")) ; } else { break ; } } } p . expect (ket) ; }
    };
}

delimited!()