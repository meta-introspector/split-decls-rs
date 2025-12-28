macro_rules! zero_sized_type {
    () => {
        # [test] fn zero_sized_type () { implement (quote ! (IFoo) , quote ! { struct Foo ; } ,) ; }
    };
}

zero_sized_type!()