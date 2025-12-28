macro_rules! thread_done {
    () => {
        pub fn thread_done () { let locals = execution (| execution | { let thread = execution . threads . active_id () ; trace ! (? thread , "thread_done: drop locals") ; execution . threads . active_mut () . drop_locals () }) ; drop (locals) ; execution (| execution | { let thread = execution . threads . active_id () ; execution . threads . active_mut () . operation = None ; execution . threads . active_mut () . set_terminated () ; let switch = execution . schedule () ; trace ! (? thread , ? switch , "thread_done: terminate") ; }) ; }
    };
}

thread_done!()