macro_rules! check_while_some {
    () => {
        # [test] fn check_while_some () { let value = (0_i32 .. 2048) . into_par_iter () . map (Some) . while_some () . max () ; assert_eq ! (value , Some (2047)) ; let counter = AtomicUsize :: new (0) ; let value = (0_i32 .. 2048) . into_par_iter () . map (| x | { counter . fetch_add (1 , Ordering :: SeqCst) ; if x < 1024 { Some (x) } else { None } }) . while_some () . max () ; assert ! (value < Some (1024)) ; assert ! (counter . load (Ordering :: SeqCst) < 2048) ; }
    };
}

check_while_some!();