macro_rules! deps {
    () => {
        ThreadPoolBuilder!();
    };
}

macro_rules! panic_thread_name {
    () => {
        deps!();
        # [test] # [cfg_attr (not (panic = "unwind") , ignore)] fn panic_thread_name () { let (start_count , start_handler) = count_handler () ; let (exit_count , exit_handler) = count_handler () ; let builder = ThreadPoolBuilder :: new () . num_threads (10) . start_handler (start_handler) . exit_handler (exit_handler) . thread_name (| i | { if i >= 5 { panic ! () ; } format ! ("panic_thread_name#{i}") }) ; let pool = crate :: unwind :: halt_unwinding (| | builder . build ()) ; assert ! (pool . is_err () , "thread-name panic should propagate!") ; assert_eq ! (5 , wait_for_counter (start_count)) ; assert_eq ! (5 , wait_for_counter (exit_count)) ; }
    };
}

panic_thread_name!();