macro_rules! is_outer {
    () => {
        fn is_outer (text : & str) -> bool { if text . starts_with ("////") || text . starts_with ("/***") { return false ; } text . starts_with ("///") || text . starts_with ("/**") }
    };
}

is_outer!()