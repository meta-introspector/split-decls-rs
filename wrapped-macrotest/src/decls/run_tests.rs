macro_rules! deps {
    () => {
        ExpansionBehavior!();
        Error!();
        ExpansionOutcome!();
    };
}

macro_rules! run_tests {
    () => {
        deps!();
        fn run_tests < I , S > (path : impl AsRef < Path > , expansion_behavior : ExpansionBehavior , args : Option < I >) where I : IntoIterator < Item = S > + Clone , S : AsRef < OsStr > , { let tests = expand_globs (& path) . into_iter () . filter (| t | ! t . test . to_string_lossy () . ends_with (EXPANDED_RS_SUFFIX)) . collect :: < Vec < _ > > () ; let len = tests . len () ; println ! ("Running {} macro expansion tests" , len) ; let project = prepare (& tests) . unwrap_or_else (| err | { panic ! ("prepare failed: {:#?}" , err) ; }) ; let mut failures = 0 ; for test in tests { let path = test . test . display () ; let expanded_path = test . test . with_extension (EXPANDED_RS_SUFFIX) ; match test . run (& project , expansion_behavior , & args) { Ok (outcome) => match outcome { ExpansionOutcome :: Same => { let _ = writeln ! (std :: io :: stdout () , "{} - ok" , path) ; } ExpansionOutcome :: Different (a , b) => { message_different (& path . to_string () , & a , & b) ; failures += 1 ; } ExpansionOutcome :: Update => { let _ = writeln ! (std :: io :: stderr () , "{} - refreshed" , expanded_path . display ()) ; } ExpansionOutcome :: ExpandError (msg) => { message_expansion_error (msg) ; failures += 1 ; } ExpansionOutcome :: NoExpandedFileFound => { let _ = writeln ! (std :: io :: stderr () , "{} is expected but not found" , expanded_path . display ()) ; failures += 1 ; } } , Err (e) => { eprintln ! ("Error: {:#?}" , e) ; failures += 1 ; } } } if failures > 0 { eprintln ! ("\n\n") ; panic ! ("{} of {} tests failed" , failures , len) ; } }
    };
}

run_tests!();