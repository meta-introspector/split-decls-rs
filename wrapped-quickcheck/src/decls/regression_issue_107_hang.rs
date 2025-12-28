macro_rules! regression_issue_107_hang {
    () => {
        # [test] # [should_panic] fn regression_issue_107_hang () { fn prop (a : Vec < u8 >) -> bool { a . contains (& 1) } quickcheck (prop as fn (_) -> bool) ; }
    };
}

regression_issue_107_hang!()