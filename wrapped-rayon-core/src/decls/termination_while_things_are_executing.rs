macro_rules! deps {
    () => {
        ThreadPoolBuilder!();
    };
}

macro_rules! termination_while_things_are_executing {
    () => {
        deps!();
        # [doc = " Test what happens when the thread pool is dropped but there are"] # [doc = " still active asynchronous tasks. We expect the thread pool to stay"] # [doc = " alive and executing until those threads are complete."] # [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn termination_while_things_are_executing () { let (tx0 , rx0) = channel () ; let (tx1 , rx1) = channel () ; { let thread_pool = ThreadPoolBuilder :: new () . build () . unwrap () ; thread_pool . spawn (move | | { let data = rx0 . recv () . unwrap () ; spawn (move | | { tx1 . send (data) . unwrap () ; }) ; }) ; } tx0 . send (22) . unwrap () ; let v = rx1 . recv () . unwrap () ; assert_eq ! (v , 22) ; }
    };
}

termination_while_things_are_executing!()