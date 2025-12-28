macro_rules! par_iter_collect_binaryheap {
    () => {
        # [test] fn par_iter_collect_binaryheap () { let a : Vec < i32 > = (0 .. 1024) . collect () ; let mut b : BinaryHeap < i32 > = a . par_iter () . cloned () . collect () ; assert_eq ! (b . peek () , Some (& 1023)) ; assert_eq ! (b . len () , 1024) ; for n in (0 .. 1024) . rev () { assert_eq ! (b . pop () , Some (n)) ; assert_eq ! (b . len () as i32 , n) ; } }
    };
}

par_iter_collect_binaryheap!();