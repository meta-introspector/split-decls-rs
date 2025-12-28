macro_rules! check_windows {
    () => {
        # [test] fn check_windows () { let a : Vec < i32 > = (0 .. 1024) . collect () ; let par : Vec < _ > = a . par_windows (2) . collect () ; let seq : Vec < _ > = a . windows (2) . collect () ; assert_eq ! (par , seq) ; let par : Vec < _ > = a . par_windows (100) . collect () ; let seq : Vec < _ > = a . windows (100) . collect () ; assert_eq ! (par , seq) ; let par : Vec < _ > = a . par_windows (1_000_000) . collect () ; let seq : Vec < _ > = a . windows (1_000_000) . collect () ; assert_eq ! (par , seq) ; let par : Vec < _ > = a . par_windows (2) . chain (a . par_windows (1_000_000)) . zip (a . par_windows (2)) . collect () ; let seq : Vec < _ > = a . windows (2) . chain (a . windows (1_000_000)) . zip (a . windows (2)) . collect () ; assert_eq ! (par , seq) ; }
    };
}

check_windows!()