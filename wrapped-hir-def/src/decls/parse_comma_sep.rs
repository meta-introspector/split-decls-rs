macro_rules! deps {
    () => {
        Literal!();
    };
}

macro_rules! parse_comma_sep {
    () => {
        deps!();
        fn parse_comma_sep < S > (iter : TtIter < '_ , S >) -> Vec < Symbol > { iter . filter_map (| tt | match tt { TtElement :: Leaf (tt :: Leaf :: Literal (tt :: Literal { kind : tt :: LitKind :: Str , symbol , .. })) => Some (symbol . clone ()) , _ => None , }) . collect () }
    };
}

parse_comma_sep!();