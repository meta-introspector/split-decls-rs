macro_rules! deps {
    () => {
        Rule!();
    };
}

macro_rules! validate_pairs {
    () => {
        deps!();
        # [doc = " It checks the parsed grammar for common mistakes:"] # [doc = " - using Pest keywords"] # [doc = " - duplicate rules"] # [doc = " - undefined rules"] # [doc = ""] # [doc = " It returns a `Result` with a `Vec` of `Error`s if any of the above is found."] # [doc = " If no errors are found, it returns the vector of names of used builtin rules."] pub fn validate_pairs (pairs : Pairs < '_ , Rule >) -> Result < Vec < & str > , Vec < Error < Rule > > > { let definitions : Vec < _ > = pairs . clone () . filter (| pair | pair . as_rule () == Rule :: grammar_rule) . map (| pair | pair . into_inner () . next () . unwrap ()) . filter (| pair | pair . as_rule () != Rule :: line_doc) . map (| pair | pair . as_span ()) . collect () ; let called_rules : Vec < _ > = pairs . clone () . filter (| pair | pair . as_rule () == Rule :: grammar_rule) . flat_map (| pair | { pair . into_inner () . flatten () . skip (1) . filter (| pair | pair . as_rule () == Rule :: identifier) . map (| pair | pair . as_span ()) }) . collect () ; let mut errors = vec ! [] ; errors . extend (validate_pest_keywords (& definitions)) ; errors . extend (validate_already_defined (& definitions)) ; errors . extend (validate_undefined (& definitions , & called_rules)) ; if ! errors . is_empty () { return Err (errors) ; } let definitions : HashSet < _ > = definitions . iter () . map (| span | span . as_str ()) . collect () ; let called_rules : HashSet < _ > = called_rules . iter () . map (| span | span . as_str ()) . collect () ; let defaults = called_rules . difference (& definitions) ; Ok (defaults . cloned () . collect ()) }
    };
}

validate_pairs!();