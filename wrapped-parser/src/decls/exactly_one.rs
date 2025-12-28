macro_rules! exactly_one {
    () => {
        pub (super) fn exactly_one < T > (iter : impl IntoIterator < Item = T >) -> T { let mut iter = iter . into_iter () ; let res = iter . next () . unwrap () ; debug_assert ! (iter . next () . is_none ()) ; res }
    };
}

exactly_one!();