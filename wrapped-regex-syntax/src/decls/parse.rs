macro_rules! deps {
    () => {
        Ast!();
        Parser!();
        Result!();
        Error!();
        ParserBuilder!();
        Hir!();
    };
}

macro_rules! parse {
    () => {
        deps!();
        # [doc = " A convenience routine for parsing a regex using default options."] # [doc = ""] # [doc = " This is equivalent to `Parser::new().parse(pattern)`."] # [doc = ""] # [doc = " If you need to set non-default options, then use a [`ParserBuilder`]."] # [doc = ""] # [doc = " This routine returns an [`Hir`](hir::Hir) value. Namely, it automatically"] # [doc = " parses the pattern as an [`Ast`](ast::Ast) and then invokes the translator"] # [doc = " to convert the `Ast` into an `Hir`. If you need access to the `Ast`, then"] # [doc = " you should use a [`ast::parse::Parser`]."] pub fn parse (pattern : & str) -> Result < hir :: Hir , Error > { Parser :: new () . parse (pattern) }
    };
}

parse!()