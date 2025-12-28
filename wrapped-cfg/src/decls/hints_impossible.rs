macro_rules! deps {
    () => {
        CfgOptions!();
    };
}

macro_rules! hints_impossible {
    () => {
        deps!();
        # [doc = " Tests that we don't suggest hints for cfgs that express an inconsistent formula."] # [test] fn hints_impossible () { let mut opts = CfgOptions :: default () ; check_enable_hints ("#![cfg(all(test, not(test)))]" , & opts , & []) ; opts . insert_atom (Symbol :: intern ("test")) ; check_enable_hints ("#![cfg(all(test, not(test)))]" , & opts , & []) ; }
    };
}

hints_impossible!();