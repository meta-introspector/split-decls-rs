macro_rules! deps {
    () => {
        ParserRule!();
        ParserNode!();
        ParserExpr!();
        RuleType!();
        Rule!();
    };
}

macro_rules! validate_tag_silent_rules {
    () => {
        deps!();
        # [cfg (feature = "grammar-extras")] fn validate_tag_silent_rules < 'a , 'i : 'a > (rules : & 'a [ParserRule < 'i >]) -> Vec < Error < Rule > > { use crate :: ast :: RuleType ; fn to_type_hash_map < 'a , 'i : 'a > (rules : & 'a [ParserRule < 'i >] ,) -> HashMap < String , (& 'a ParserNode < 'i > , RuleType) > { rules . iter () . map (| r | (r . name . clone () , (& r . node , r . ty))) . collect () } let mut result = vec ! [] ; fn check_silent_builtin < 'a , 'i : 'a > (expr : & ParserExpr < 'i > , rules_ref : & HashMap < String , (& 'a ParserNode < 'i > , RuleType) > , span : Span < 'a > ,) -> Option < Error < Rule > > { match & expr { ParserExpr :: Ident (rule_name) => { let rule = rules_ref . get (rule_name) ; if matches ! (rule , Some ((_ , RuleType :: Silent))) { return Some (Error :: < Rule > :: new_from_span (ErrorVariant :: CustomError { message : "tags on silent rules will not appear in the output" . to_owned () , } , span ,)) ; } else if BUILTINS . contains (rule_name . as_str ()) { return Some (Error :: new_from_span (ErrorVariant :: CustomError { message : "tags on built-in rules will not appear in the output" . to_owned () , } , span ,)) ; } } ParserExpr :: Rep (node) | ParserExpr :: RepMinMax (node , _ , _) | ParserExpr :: RepMax (node , _) | ParserExpr :: RepMin (node , _) | ParserExpr :: RepOnce (node) | ParserExpr :: RepExact (node , _) | ParserExpr :: Opt (node) | ParserExpr :: Push (node) | ParserExpr :: PosPred (node) | ParserExpr :: NegPred (node) => { return check_silent_builtin (& node . expr , rules_ref , span) ; } _ => { } } ; None } let rules_map = to_type_hash_map (rules) ; for rule in rules { let rules_ref = & rules_map ; let mut errors = rule . node . clone () . filter_map_top_down (| node1 | { if let ParserExpr :: NodeTag (node2 , _) = node1 . expr { check_silent_builtin (& node2 . expr , rules_ref , node1 . span) } else { None } }) ; result . append (& mut errors) ; } result }
    };
}

validate_tag_silent_rules!();