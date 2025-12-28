macro_rules! test_finish_is_consistent {
    () => {
        fn test_finish_is_consistent < T : Hasher > (constructor : impl Fn (u128 , u128) -> T) { let mut hasher = constructor (1 , 2) ; "Foo" . hash (& mut hasher) ; let a = hasher . finish () ; let b = hasher . finish () ; assert_eq ! (a , b) ; }
    };
}

test_finish_is_consistent!()