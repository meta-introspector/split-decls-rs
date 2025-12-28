macro_rules! check_vec_deque {
    () => {
        # [test] fn check_vec_deque () { use std :: collections :: VecDeque ; let mut a : VecDeque < i32 > = (0 .. 10) . collect () ; a . drain (.. 5) ; a . extend (0 .. 5) ; assert_eq ! (45 , a . par_iter () . sum ::< i32 > ()) ; a . par_iter_mut () . for_each (| x | * x = - * x) ; assert_eq ! (- 45 , a . into_par_iter () . sum ::< i32 > ()) ; }
    };
}

check_vec_deque!()