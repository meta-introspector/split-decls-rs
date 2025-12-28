macro_rules! check_btree_set {
    () => {
        # [test] fn check_btree_set () { use std :: collections :: BTreeSet ; let a : BTreeSet < i32 > = (0 .. 10) . collect () ; assert_eq ! (45 , a . par_iter () . sum ::< i32 > ()) ; assert_eq ! (45 , a . into_par_iter () . sum ::< i32 > ()) ; }
    };
}

check_btree_set!()