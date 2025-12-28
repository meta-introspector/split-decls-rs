macro_rules! deps {
    () => {
        WildStr!();
    };
}

macro_rules! test {
    () => {
        deps!();
        # [cfg (test)] mod test { use snapbox :: assert_data_eq ; use snapbox :: prelude :: * ; use snapbox :: str ; use super :: * ; # [test] fn wild_str_cmp () { for (a , b) in & [("a b" , "a b") , ("a[..]b" , "a b") , ("a[..]" , "a b") , ("[..]" , "a b") , ("[..]b" , "a b") ,] { assert_eq ! (WildStr :: new (a) , b) ; } for (a , b) in & [("[..]b" , "c") , ("b" , "c") , ("b" , "cb")] { assert_ne ! (WildStr :: new (a) , b) ; } } # [test] fn redact_elapsed_time () { let mut subs = snapbox :: Redactions :: new () ; add_regex_redactions (& mut subs) ; assert_data_eq ! (subs . redact ("[FINISHED] `release` profile [optimized] target(s) in 5.5s") , str ! ["[FINISHED] `release` profile [optimized] target(s) in [ELAPSED]s"] . raw ()) ; assert_data_eq ! (subs . redact ("[FINISHED] `release` profile [optimized] target(s) in 1m 05s") , str ! ["[FINISHED] `release` profile [optimized] target(s) in [ELAPSED]s"] . raw ()) ; } }
    };
}

test!();