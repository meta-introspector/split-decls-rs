macro_rules! between {
    () => {
        fn between < 'a > (begin : ParseStream < 'a > , end : ParseStream < 'a >) -> TokenStream { let end = end . cursor () ; let mut cursor = begin . cursor () ; let mut tokens = TokenStream :: new () ; while cursor < end { let (tt , next) = cursor . token_tree () . unwrap () ; if end < next { if let Some ((inside , _span , _after)) = cursor . group (Delimiter :: None) { cursor = inside ; continue ; } if tokens . is_empty () { tokens . extend (iter :: once (tt)) ; } break ; } tokens . extend (iter :: once (tt)) ; cursor = next ; } tokens }
    };
}

between!();