macro_rules! deps {
    () => {
        Align!();
    };
}

macro_rules! align_constants {
    () => {
        deps!();
        # [test] fn align_constants () { assert_eq ! (Align :: ONE , Align :: from_bytes (1) . unwrap ()) ; assert_eq ! (Align :: EIGHT , Align :: from_bytes (8) . unwrap ()) ; }
    };
}

align_constants!()