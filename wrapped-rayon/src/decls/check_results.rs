macro_rules! check_results {
    () => {
        # [test] fn check_results () { let mut a = vec ! [Err (()) , Ok (1i32) , Err (()) , Err (()) , Ok (2) , Ok (4)] ; assert_eq ! (7 , a . par_iter () . flat_map (| res | res) . sum ::< i32 > ()) ; assert_eq ! (Err ::< i32 , () > (()) , a . par_iter () . cloned () . sum ()) ; assert_eq ! (Ok (7) , a . par_iter () . cloned () . filter (Result :: is_ok) . sum ()) ; assert_eq ! (Err ::< i32 , () > (()) , a . par_iter () . cloned () . product ()) ; assert_eq ! (Ok (8) , a . par_iter () . cloned () . filter (Result :: is_ok) . product ()) ; a . par_iter_mut () . flat_map (| res | res) . for_each (| x | * x = * x * * x) ; assert_eq ! (21 , a . into_par_iter () . flat_map (| res | res) . sum ::< i32 > ()) ; }
    };
}

check_results!();