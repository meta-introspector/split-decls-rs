macro_rules! try_eat_dollar {
    () => {
        fn try_eat_dollar (src : & mut TtIter < '_ , Span >) -> bool { if let Some (TtElement :: Leaf (tt :: Leaf :: Punct (tt :: Punct { char : '$' , .. }))) = src . peek () { let _ = src . next () ; return true ; } false }
    };
}

try_eat_dollar!()