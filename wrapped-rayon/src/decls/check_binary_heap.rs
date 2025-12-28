macro_rules! check_binary_heap {
    () => {
        # [test] fn check_binary_heap () { use std :: collections :: BinaryHeap ; let a : BinaryHeap < i32 > = (0 .. 10) . collect () ; assert_eq ! (45 , a . par_iter () . sum ::< i32 > ()) ; assert_eq ! (45 , a . into_par_iter () . sum ::< i32 > ()) ; }
    };
}

check_binary_heap!()