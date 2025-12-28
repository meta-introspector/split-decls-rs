macro_rules! check_options {
    () => {
        # [test] fn check_options () { let mut a = vec ! [None , Some (1) , None , None , Some (2) , Some (4)] ; assert_eq ! (7 , a . par_iter () . flat_map (| opt | opt) . sum ::< i32 > ()) ; assert_eq ! (7 , a . par_iter () . flat_map (| opt | opt) . sum ::< i32 > ()) ; a . par_iter_mut () . flat_map (| opt | opt) . for_each (| x | * x = * x * * x) ; assert_eq ! (21 , a . into_par_iter () . flat_map (| opt | opt) . sum ::< i32 > ()) ; }
    };
}

check_options!();