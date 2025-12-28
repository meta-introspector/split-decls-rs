macro_rules! compute_ws {
    () => {
        fn compute_ws (left : SyntaxKind , right : SyntaxKind) -> & 'static str { match left { T ! ['('] | T ! ['['] => return "" , T ! ['{'] => { if let USE_TREE = right { return "" ; } } _ => () , } match right { T ! [')'] | T ! [']'] => return "" , T ! ['}'] => { if let USE_TREE = left { return "" ; } } T ! [.] => return "" , _ => () , } " " }
    };
}

compute_ws!();