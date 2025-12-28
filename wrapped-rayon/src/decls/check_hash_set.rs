macro_rules! check_hash_set {
    () => {
        # [test] fn check_hash_set () { use std :: collections :: HashSet ; let a : HashSet < i32 > = (0 .. 10) . collect () ; assert_eq ! (45 , a . par_iter () . sum ::< i32 > ()) ; assert_eq ! (45 , a . into_par_iter () . sum ::< i32 > ()) ; }
    };
}

check_hash_set!();