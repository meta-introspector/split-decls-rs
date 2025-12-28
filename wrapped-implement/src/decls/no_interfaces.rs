macro_rules! no_interfaces {
    () => {
        # [test] fn no_interfaces () { implement (quote ! () , quote ! { struct Foo { } } ,) ; }
    };
}

no_interfaces!();