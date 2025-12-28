macro_rules! parse_expr {
    () => {
        # [doc = " Parse an expression. On error, emit it, advancing to `Eof`, and return `Err`."] pub (crate) fn parse_expr (p : & mut parser :: Parser < '_ >) -> Result < Box < ast :: Expr > , ErrorGuaranteed > { let guar = match p . parse_expr () { Ok (expr) => return Ok (expr) , Err (err) => err . emit () , } ; while p . token != token :: Eof { p . bump () ; } Err (guar) }
    };
}

parse_expr!()