macro_rules! map_reduce {
    () => {
        # [test] fn map_reduce () { let a : Vec < i32 > = (0 .. 1024) . collect () ; let r1 = a . par_iter () . map (| & i | i + 1) . reduce (| | 0 , | i , j | i + j) ; let r2 = a . iter () . map (| & i | i + 1) . sum () ; assert_eq ! (r1 , r2) ; }
    };
}

map_reduce!()