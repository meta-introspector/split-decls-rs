macro_rules! check_chunks_mut {
    () => {
        # [test] fn check_chunks_mut () { let mut a : Vec < i32 > = vec ! [1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10] ; let mut b : Vec < i32 > = a . clone () ; a . par_chunks_mut (2) . for_each (| c | c [0] = c . iter () . sum ()) ; b . chunks_mut (2) . for_each (| c | c [0] = c . iter () . sum ()) ; assert_eq ! (a , & [3 , 2 , 7 , 4 , 11 , 6 , 15 , 8 , 19 , 10]) ; assert_eq ! (a , b) ; let mut a : Vec < i32 > = vec ! [1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10] ; let mut b : Vec < i32 > = a . clone () ; a . par_chunks_mut (3) . for_each (| c | c [0] = c . iter () . sum ()) ; b . chunks_mut (3) . for_each (| c | c [0] = c . iter () . sum ()) ; assert_eq ! (a , & [6 , 2 , 3 , 15 , 5 , 6 , 24 , 8 , 9 , 10]) ; assert_eq ! (a , b) ; }
    };
}

check_chunks_mut!();