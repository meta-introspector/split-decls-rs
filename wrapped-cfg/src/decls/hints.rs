macro_rules! deps {
    () => {
        CfgOptions!();
    };
}

macro_rules! hints {
    () => {
        deps!();
        # [test] fn hints () { let mut opts = CfgOptions :: default () ; check_enable_hints ("#![cfg(test)]" , & opts , & ["enable test"]) ; check_enable_hints ("#![cfg(not(test))]" , & opts , & []) ; check_enable_hints ("#![cfg(any(a, b))]" , & opts , & ["enable a" , "enable b"]) ; check_enable_hints ("#![cfg(any(b, a))]" , & opts , & ["enable b" , "enable a"]) ; check_enable_hints ("#![cfg(all(a, b))]" , & opts , & ["enable a and b"]) ; opts . insert_atom (Symbol :: intern ("test")) ; check_enable_hints ("#![cfg(test)]" , & opts , & []) ; check_enable_hints ("#![cfg(not(test))]" , & opts , & ["disable test"]) ; }
    };
}

hints!();