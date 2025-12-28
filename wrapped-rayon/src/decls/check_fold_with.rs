macro_rules! check_fold_with {
    () => {
        # [test] fn check_fold_with () { let (sender , receiver) = mpsc :: channel () ; let a : HashSet < _ > = (0 .. 1024) . collect () ; a . par_iter () . cloned () . fold_with (sender , | s , i | { s . send (i) . unwrap () ; s }) . count () ; let b : HashSet < _ > = receiver . iter () . collect () ; assert_eq ! (a , b) ; }
    };
}

check_fold_with!()