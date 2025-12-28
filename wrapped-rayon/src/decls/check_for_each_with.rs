macro_rules! check_for_each_with {
    () => {
        # [test] fn check_for_each_with () { let (sender , receiver) = mpsc :: channel () ; let a : HashSet < _ > = (0 .. 1024) . collect () ; a . par_iter () . cloned () . for_each_with (sender , | s , i | s . send (i) . unwrap ()) ; let b : HashSet < _ > = receiver . iter () . collect () ; assert_eq ! (a , b) ; }
    };
}

check_for_each_with!();