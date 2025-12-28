macro_rules! deps {
    () => {
        ThreadPoolBuilder!();
    };
}

macro_rules! linear_stack_growth {
    () => {
        deps!();
        # [doc = " Check that if you have a chain of scoped tasks where T0 spawns T1"] # [doc = " spawns T2 and so forth down to Tn, the stack space should not grow"] # [doc = " linearly with N. We test this by some unsafe hackery and"] # [doc = " permitting an approx 10% change with a 10x input change."] # [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn linear_stack_growth () { let builder = ThreadPoolBuilder :: new () . num_threads (1) ; let pool = builder . build () . unwrap () ; pool . install (| | { let mut max_diff = Mutex :: new (0) ; let bottom_of_stack = 0 ; scope (| s | the_final_countdown (s , & bottom_of_stack , & max_diff , 5)) ; let diff_when_5 = * max_diff . get_mut () . unwrap () as f64 ; scope (| s | the_final_countdown (s , & bottom_of_stack , & max_diff , 500)) ; let diff_when_500 = * max_diff . get_mut () . unwrap () as f64 ; let ratio = diff_when_5 / diff_when_500 ; assert ! (ratio > 0.9 && ratio < 1.1 , "stack usage ratio out of bounds: {ratio}") ; }) ; }
    };
}

linear_stack_growth!();