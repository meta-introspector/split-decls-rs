macro_rules! simple_type {
    () => {
        # [test] fn simple_type () { implement (quote ! (IFoo) , quote ! { struct Foo { x : u32 , } } ,) ; }
    };
}

simple_type!();