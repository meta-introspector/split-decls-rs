macro_rules! check_find_not_present {
    () => {
        # [test] fn check_find_not_present () { let counter = AtomicUsize :: new (0) ; let value : Option < i32 > = (0_i32 .. 2048) . into_par_iter () . find_any (| & p | { counter . fetch_add (1 , Ordering :: SeqCst) ; p >= 2048 }) ; assert ! (value . is_none ()) ; assert ! (counter . load (Ordering :: SeqCst) == 2048) ; }
    };
}

check_find_not_present!();