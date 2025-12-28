macro_rules! if_checking_recursion_limit {
    () => {
        # [cfg (feature = "unbounded_depth")] macro_rules ! if_checking_recursion_limit { ($ this : ident $ ($ body : tt) *) => { if !$ this . disable_recursion_limit { $ this $ ($ body) * } } ; }
    };
}

if_checking_recursion_limit!()