macro_rules! check_extend_heap {
    () => {
        # [test] fn check_extend_heap () { let mut serial : BinaryHeap < _ > = Default :: default () ; let mut parallel : BinaryHeap < _ > = Default :: default () ; let v : Vec < _ > = (0 .. 128) . collect () ; serial . extend (& v) ; parallel . par_extend (& v) ; assert_eq ! (serial . clone () . into_sorted_vec () , parallel . clone () . into_sorted_vec ()) ; serial . extend (- 128 .. 0) ; parallel . par_extend (- 128 .. 0) ; assert_eq ! (serial . into_sorted_vec () , parallel . into_sorted_vec ()) ; }
    };
}

check_extend_heap!()