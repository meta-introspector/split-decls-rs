macro_rules! replace_macro_invocations {
    () => {
        # [test] fn replace_macro_invocations () { assert_ssr_transform ("try_!($a) ==>> $a?" , "macro_rules! try_ {() => {}} fn f1() -> Result<(), E> {bar(try_!(foo()));}" , expect ! [["macro_rules! try_ {() => {}} fn f1() -> Result<(), E> {bar(foo()?);}"]] ,) ; }
    };
}

replace_macro_invocations!();