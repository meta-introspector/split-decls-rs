macro_rules! extract_directive_call_path {
    () => {
        fn extract_directive_call_path (directive : & Expr) -> Option < syn :: Path > { if let Expr :: Call (expr) = directive { if let Expr :: Path (ref expr) = * expr . func { let mut path = expr . path . clone () ; if path . segments . pop () ? . value () . ident != "apply" { return None ; } path . segments . pop_punct () ? ; return Some (path) ; } } None }
    };
}

extract_directive_call_path!();