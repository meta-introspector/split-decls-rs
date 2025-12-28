macro_rules! check_repeat_unbounded {
    () => {
        # [test] # [ignore] # [should_panic (expected = "overflow")] # [cfg (debug_assertions)] fn check_repeat_unbounded () { let pool = ThreadPoolBuilder :: new () . num_threads (1) . build () . unwrap () ; pool . install (| | { println ! ("counted {} repeats" , repeat (()) . count ()) ; }) ; }
    };
}

check_repeat_unbounded!();