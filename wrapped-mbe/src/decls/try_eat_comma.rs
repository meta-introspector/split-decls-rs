macro_rules! try_eat_comma {
    () => {
        fn try_eat_comma (src : & mut TtIter < '_ , Span >) -> bool { if let Some (TtElement :: Leaf (tt :: Leaf :: Punct (tt :: Punct { char : ',' , .. }))) = src . peek () { let _ = src . next () ; return true ; } false }
    };
}

try_eat_comma!()