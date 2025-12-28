macro_rules! deps {
    () => {
        SearchTest!();
    };
}

macro_rules! search_tests_have_unique_names {
    () => {
        deps!();
        # [test] fn search_tests_have_unique_names () { let assert = | constname , tests : & [SearchTest] | { let mut seen = HashMap :: new () ; for (i , test) in tests . iter () . enumerate () { if ! seen . contains_key (test . name) { seen . insert (test . name , i) ; } else { let last = seen [test . name] ; panic ! ("{} tests have duplicate names at positions {} and {}" , constname , last , i) ; } } } ; assert ("BASICS" , BASICS) ; assert ("STANDARD" , STANDARD) ; assert ("LEFTMOST" , LEFTMOST) ; assert ("LEFTMOST_FIRST" , LEFTMOST_FIRST) ; assert ("LEFTMOST_LONGEST" , LEFTMOST_LONGEST) ; assert ("NON_OVERLAPPING" , NON_OVERLAPPING) ; assert ("OVERLAPPING" , OVERLAPPING) ; assert ("REGRESSION" , REGRESSION) ; }
    };
}

search_tests_have_unique_names!()