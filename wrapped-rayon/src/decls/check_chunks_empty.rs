macro_rules! check_chunks_empty {
    () => {
        # [test] fn check_chunks_empty () { let v : Vec < i32 > = vec ! [] ; let expected : Vec < Vec < i32 > > = vec ! [] ; assert_eq ! (expected , v . into_par_iter () . chunks (2) . collect ::< Vec < Vec < i32 >>> ()) ; }
    };
}

check_chunks_empty!()