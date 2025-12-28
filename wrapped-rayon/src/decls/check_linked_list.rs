macro_rules! check_linked_list {
    () => {
        # [test] fn check_linked_list () { use std :: collections :: LinkedList ; let mut a : LinkedList < i32 > = (0 .. 10) . collect () ; assert_eq ! (45 , a . par_iter () . sum ::< i32 > ()) ; a . par_iter_mut () . for_each (| x | * x = - * x) ; assert_eq ! (- 45 , a . into_par_iter () . sum ::< i32 > ()) ; }
    };
}

check_linked_list!()