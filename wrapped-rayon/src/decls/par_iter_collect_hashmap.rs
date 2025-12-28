macro_rules! par_iter_collect_hashmap {
    () => {
        # [test] fn par_iter_collect_hashmap () { let a : Vec < i32 > = (0 .. 1024) . collect () ; let b : HashMap < i32 , String > = a . par_iter () . map (| & i | (i , format ! ("{i}"))) . collect () ; assert_eq ! (& b [& 3] , "3") ; assert_eq ! (b . len () , 1024) ; }
    };
}

par_iter_collect_hashmap!()