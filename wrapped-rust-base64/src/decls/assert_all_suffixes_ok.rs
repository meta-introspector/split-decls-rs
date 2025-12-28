macro_rules! deps {
    () => {
        Engine!();
    };
}

macro_rules! assert_all_suffixes_ok {
    () => {
        deps!();
        fn assert_all_suffixes_ok < E : Engine > (engine : E , suffixes : Vec < & str >) { for num_prefix_quads in 0 .. 256 { for & suffix in suffixes . iter () { let mut encoded = "AAAA" . repeat (num_prefix_quads) ; encoded . push_str (suffix) ; let res = & engine . decode (& encoded) ; assert ! (res . is_ok ()) ; } } }
    };
}

assert_all_suffixes_ok!()