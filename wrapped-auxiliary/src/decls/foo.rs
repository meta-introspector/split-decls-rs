macro_rules! foo {
    () => {
        pub fn foo () { println ! ("x") ; let mut map = HashMap :: new () ; map . insert (1 , "foo") ; }
    };
}

foo!()