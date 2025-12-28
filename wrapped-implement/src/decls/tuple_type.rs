macro_rules! tuple_type {
    () => {
        # [test] fn tuple_type () { implement (quote ! (IFoo) , quote ! { struct Foo (pub i32) ; } ,) ; }
    };
}

tuple_type!()