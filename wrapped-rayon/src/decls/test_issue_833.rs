macro_rules! test_issue_833 {
    () => {
        # [test] fn test_issue_833 () { fn is_even (n : i64) -> bool { n % 2 == 0 } let v : Vec < _ > = (1 ..= 100) . into_par_iter () . filter (| & x | is_even (x)) . collect () ; assert ! (v . into_iter () . eq ((2 ..= 100) . step_by (2))) ; let pos = (0 ..= 100) . into_par_iter () . position_any (| x | x == 50i16) ; assert_eq ! (pos , Some (50usize)) ; assert ! ((0 ..= 100) . into_par_iter () . zip (0 ..= 100) . all (| (a , b) | i16 :: eq (& a , & b))) ; }
    };
}

test_issue_833!()