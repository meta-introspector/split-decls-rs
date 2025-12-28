macro_rules! par_iter_collect_btreeset {
    () => {
        # [test] fn par_iter_collect_btreeset () { let a : Vec < i32 > = (0 .. 1024) . collect () ; let b : BTreeSet < i32 > = a . par_iter () . cloned () . collect () ; assert_eq ! (b . len () , 1024) ; }
    };
}

par_iter_collect_btreeset!();