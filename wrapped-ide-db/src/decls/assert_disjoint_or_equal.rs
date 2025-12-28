macro_rules! deps {
    () => {
        Indel!();
    };
}

macro_rules! assert_disjoint_or_equal {
    () => {
        deps!();
        fn assert_disjoint_or_equal (indels : & mut [Indel]) { assert ! (check_disjoint_and_sort (indels)) ; }
    };
}

assert_disjoint_or_equal!();