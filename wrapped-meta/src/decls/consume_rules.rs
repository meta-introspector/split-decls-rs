macro_rules! deps {
    () => {
        Rule!();
    };
}

macro_rules! consume_rules {
    () => {
        deps!();
        # [doc = " Converts a parser's result (`Pairs`) to an AST"] pub fn consume_rules (pairs : Pairs < '_ , Rule >) -> Result < Vec < AstRule > , Vec < Error < Rule > > > { let rules = consume_rules_with_spans (pairs) ? ; let errors = validator :: validate_ast (& rules) ; if errors . is_empty () { Ok (rules . into_iter () . map (convert_rule) . collect ()) } else { Err (errors) } }
    };
}

consume_rules!();