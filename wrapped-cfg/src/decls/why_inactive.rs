macro_rules! deps {
    () => {
        CfgOptions!();
    };
}

macro_rules! why_inactive {
    () => {
        deps!();
        # [test] fn why_inactive () { let mut opts = CfgOptions :: default () ; opts . insert_atom (Symbol :: intern ("test")) ; opts . insert_atom (Symbol :: intern ("test2")) ; check_why_inactive ("#![cfg(a)]" , & opts , expect ! [["a is disabled"]]) ; check_why_inactive ("#![cfg(not(test))]" , & opts , expect ! [["test is enabled"]]) ; check_why_inactive ("#![cfg(all(not(test), not(test2)))]" , & opts , expect ! [["test and test2 are enabled"]] ,) ; check_why_inactive ("#![cfg(all(a, b))]" , & opts , expect ! [["a and b are disabled"]]) ; check_why_inactive ("#![cfg(all(not(test), a))]" , & opts , expect ! [["test is enabled and a is disabled"]] ,) ; check_why_inactive ("#![cfg(all(not(test), test2, a))]" , & opts , expect ! [["test is enabled and a is disabled"]] ,) ; check_why_inactive ("#![cfg(all(not(test), not(test2), a))]" , & opts , expect ! [["test and test2 are enabled and a is disabled"]] ,) ; }
    };
}

why_inactive!();