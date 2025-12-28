macro_rules! check_skip {
    () => {
        # [test] fn check_skip () { let a : Vec < usize > = (0 .. 1024) . collect () ; let mut v1 = Vec :: new () ; a . par_iter () . skip (16) . collect_into_vec (& mut v1) ; let v2 = a . iter () . skip (16) . collect :: < Vec < _ > > () ; assert_eq ! (v1 , v2) ; let mut v1 = Vec :: new () ; a . par_iter () . skip (2048) . collect_into_vec (& mut v1) ; let v2 = a . iter () . skip (2048) . collect :: < Vec < _ > > () ; assert_eq ! (v1 , v2) ; let mut v1 = Vec :: new () ; a . par_iter () . skip (0) . collect_into_vec (& mut v1) ; # [allow (clippy :: iter_skip_zero)] let v2 = a . iter () . skip (0) . collect :: < Vec < _ > > () ; assert_eq ! (v1 , v2) ; use std :: sync :: atomic :: { AtomicUsize , Ordering } ; let num = AtomicUsize :: new (0) ; a . par_iter () . map (| & n | num . fetch_add (n , Ordering :: Relaxed)) . skip (512) . count () ; assert_eq ! (num . load (Ordering :: Relaxed) , a . iter () . sum ::< usize > ()) ; }
    };
}

check_skip!();