// Generated macro for consume_rules_with_spans (function)
macro_rules! Depcrate_parserconsume_rules_with_spans {
() => {
// Module: crate::parser
// Provides: {"consume_rules_with_spans"}
// Dependencies: {}
fn consume_rules_with_spans (pairs : Pairs < '_ , Rule > ,) -> Result < Vec < ParserRule < '_ > > , Vec < Error < Rule > > > { let pratt = PrattParser :: new () . op (Op :: infix (Rule :: choice_operator , Assoc :: Left)) . op (Op :: infix (Rule :: sequence_operator , Assoc :: Left)) ; pairs . filter (| pair | pair . as_rule () == Rule :: grammar_rule) . filter (| pair | { let mut pairs = pair . clone () . into_inner () ; let pair = pairs . next () . unwrap () ; pair . as_rule () != Rule :: line_doc }) . map (| pair | { let mut pairs = pair . into_inner () . peekable () ; let span = pairs . next () . unwrap () . as_span () ; let name = span . as_str () . to_owned () ; pairs . next () . unwrap () ; let ty = if pairs . peek () . unwrap () . as_rule () != Rule :: opening_brace { match pairs . next () . unwrap () . as_rule () { Rule :: silent_modifier => RuleType :: Silent , Rule :: atomic_modifier => RuleType :: Atomic , Rule :: compound_atomic_modifier => RuleType :: CompoundAtomic , Rule :: non_atomic_modifier => RuleType :: NonAtomic , _ => unreachable ! () , } } else { RuleType :: Normal } ; pairs . next () . unwrap () ; let mut inner_nodes = pairs . next () . unwrap () . into_inner () . peekable () ; if inner_nodes . peek () . unwrap () . as_rule () == Rule :: choice_operator { inner_nodes . next () . unwrap () ; } let node = consume_expr (inner_nodes , & pratt) ? ; Ok (ParserRule { name , span , ty , node , }) }) . collect () }
};
}
