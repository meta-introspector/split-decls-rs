macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! issue_5 {
    () => {
        deps!();
        # [test] fn issue_5 () { assert ! (Some (SmallVec ::<& u32 , 2 >:: new ()) . is_some ()) ; }
    };
}

issue_5!();