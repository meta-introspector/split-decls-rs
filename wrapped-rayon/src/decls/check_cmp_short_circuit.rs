macro_rules! check_cmp_short_circuit {
    () => {
        # [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn check_cmp_short_circuit () { let pool = ThreadPoolBuilder :: new () . num_threads (1) . build () . unwrap () ; let a = vec ! [0 ; 1024] ; let mut b = a . clone () ; b [42] = 1 ; pool . install (| | { let expected = :: std :: cmp :: Ordering :: Less ; assert_eq ! (a . par_iter () . cmp (& b) , expected) ; for len in 1 .. 10 { let counter = AtomicUsize :: new (0) ; let result = a . par_iter () . with_max_len (len) . inspect (| _ | { counter . fetch_add (1 , Ordering :: SeqCst) ; }) . cmp (& b) ; assert_eq ! (result , expected) ; assert ! (counter . into_inner () < a . len ()) ; } }) ; }
    };
}

check_cmp_short_circuit!();