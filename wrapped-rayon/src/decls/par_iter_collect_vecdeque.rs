macro_rules! par_iter_collect_vecdeque {
    () => {
        # [test] fn par_iter_collect_vecdeque () { let a : Vec < i32 > = (0 .. 1024) . collect () ; let b : VecDeque < i32 > = a . par_iter () . cloned () . collect () ; let c : VecDeque < i32 > = a . iter () . cloned () . collect () ; assert_eq ! (b , c) ; }
    };
}

par_iter_collect_vecdeque!()