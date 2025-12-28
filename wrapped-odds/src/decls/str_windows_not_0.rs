macro_rules! deps {
    () => {
        CharWindows!();
    };
}

macro_rules! str_windows_not_0 {
    () => {
        deps!();
        # [test] # [should_panic] fn str_windows_not_0 () { CharWindows :: new ("abc" , 0) ; }
    };
}

str_windows_not_0!()