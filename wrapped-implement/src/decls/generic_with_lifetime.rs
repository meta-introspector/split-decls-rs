macro_rules! generic_with_lifetime {
    () => {
        # [test] fn generic_with_lifetime () { implement (quote ! () , quote ! { pub struct Foo <'a > { pub x : &'a [u8] , } } ,) ; }
    };
}

generic_with_lifetime!()