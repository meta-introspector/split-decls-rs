macro_rules! deps {
    () => {
        ParserExpr!();
        ParserNode!();
    };
}

macro_rules! is_non_progressing {
    () => {
        deps!();
        # [doc = " Checks if `expr` is non-progressing, that is the expression does not"] # [doc = " consume any input or any stack. This includes expressions matching the empty input,"] # [doc = " `SOI` and \u{300} `EOI`, predicates and repetitions."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```pest"] # [doc = " not_progressing_1 = { \"\" }"] # [doc = " not_progressing_2 = { \"a\"? }"] # [doc = " not_progressing_3 = { !\"a\" }"] # [doc = " ```"] # [doc = ""] # [doc = " # Assumptions"] # [doc = " - In `ParserExpr::RepMinMax(inner,min,max)`, `min<=max`"] # [doc = " - All rules identifiers have a matching definition"] # [doc = " - There is no left-recursion (if only this one is broken returns false)"] # [doc = " - Every expression is being checked"] fn is_non_progressing < 'i > (expr : & ParserExpr < 'i > , rules : & HashMap < String , & ParserNode < 'i > > , trace : & mut Vec < String > ,) -> bool { match * expr { ParserExpr :: Str (ref string) | ParserExpr :: Insens (ref string) => string . is_empty () , ParserExpr :: Ident (ref ident) => { if ident == "SOI" || ident == "EOI" { return true ; } if ! trace . contains (ident) { if let Some (node) = rules . get (ident) { trace . push (ident . clone ()) ; let result = is_non_progressing (& node . expr , rules , trace) ; trace . pop () . unwrap () ; return result ; } } false } ParserExpr :: Seq (ref lhs , ref rhs) => { is_non_progressing (& lhs . expr , rules , trace) && is_non_progressing (& rhs . expr , rules , trace) } ParserExpr :: Choice (ref lhs , ref rhs) => { is_non_progressing (& lhs . expr , rules , trace) || is_non_progressing (& rhs . expr , rules , trace) } ParserExpr :: PosPred (_) | ParserExpr :: NegPred (_) => true , ParserExpr :: Rep (_) | ParserExpr :: Opt (_) | ParserExpr :: RepMax (_ , _) => true , ParserExpr :: Range (_ , _) => false , ParserExpr :: PeekSlice (_ , _) => { false } ParserExpr :: RepExact (ref inner , min) | ParserExpr :: RepMin (ref inner , min) | ParserExpr :: RepMinMax (ref inner , min , _) => { min == 0 || is_non_progressing (& inner . expr , rules , trace) } ParserExpr :: Push (ref inner) => is_non_progressing (& inner . expr , rules , trace) , # [cfg (feature = "grammar-extras")] ParserExpr :: PushLiteral (_) => true , ParserExpr :: RepOnce (ref inner) => is_non_progressing (& inner . expr , rules , trace) , # [cfg (feature = "grammar-extras")] ParserExpr :: NodeTag (ref inner , _) => is_non_progressing (& inner . expr , rules , trace) , } }
    };
}

is_non_progressing!()