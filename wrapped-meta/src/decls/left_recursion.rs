macro_rules! deps {
    () => {
        ParserExpr!();
        Rule!();
        ParserNode!();
    };
}

macro_rules! left_recursion {
    () => {
        deps!();
        fn left_recursion < 'a , 'i : 'a > (rules : HashMap < String , & 'a ParserNode < 'i > >) -> Vec < Error < Rule > > { fn check_expr < 'a , 'i : 'a > (node : & 'a ParserNode < 'i > , rules : & 'a HashMap < String , & ParserNode < 'i > > , trace : & mut Vec < String > ,) -> Option < Error < Rule > > { match node . expr . clone () { ParserExpr :: Ident (other) => { if trace [0] == other { trace . push (other) ; let chain = trace . iter () . map (| ident | ident . as_ref ()) . collect :: < Vec < _ > > () . join (" -> ") ; return Some (Error :: new_from_span (ErrorVariant :: CustomError { message : format ! ("rule {} is left-recursive ({}); pest::pratt_parser might be useful \
                                 in this case" , node . span . as_str () , chain) } , node . span)) ; } if ! trace . contains (& other) { if let Some (node) = rules . get (& other) { trace . push (other) ; let result = check_expr (node , rules , trace) ; trace . pop () . unwrap () ; return result ; } } None } ParserExpr :: Seq (ref lhs , ref rhs) => { if is_non_failing (& lhs . expr , rules , & mut vec ! [trace . last () . unwrap () . clone ()]) || is_non_progressing (& lhs . expr , rules , & mut vec ! [trace . last () . unwrap () . clone ()] ,) { check_expr (rhs , rules , trace) } else { check_expr (lhs , rules , trace) } } ParserExpr :: Choice (ref lhs , ref rhs) => { check_expr (lhs , rules , trace) . or_else (| | check_expr (rhs , rules , trace)) } ParserExpr :: Rep (ref node) => check_expr (node , rules , trace) , ParserExpr :: RepOnce (ref node) => check_expr (node , rules , trace) , ParserExpr :: Opt (ref node) => check_expr (node , rules , trace) , ParserExpr :: PosPred (ref node) => check_expr (node , rules , trace) , ParserExpr :: NegPred (ref node) => check_expr (node , rules , trace) , ParserExpr :: Push (ref node) => check_expr (node , rules , trace) , _ => None , } } let mut errors = vec ! [] ; for (name , node) in & rules { let name = name . clone () ; if let Some (error) = check_expr (node , & rules , & mut vec ! [name]) { errors . push (error) ; } } errors }
    };
}

left_recursion!();