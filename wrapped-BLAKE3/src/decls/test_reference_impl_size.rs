macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! test_reference_impl_size {
    () => {
        deps!();
        # [test] fn test_reference_impl_size () { assert_eq ! (1880 , core :: mem :: size_of ::< reference_impl :: Hasher > ()) ; }
    };
}

test_reference_impl_size!();