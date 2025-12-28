macro_rules! deps {
    () => {
        ThreadPoolBuilder!();
    };
}

macro_rules! check_config_build {
    () => {
        deps!();
        # [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn check_config_build () { let pool = ThreadPoolBuilder :: new () . num_threads (22) . build () . unwrap () ; assert_eq ! (pool . current_num_threads () , 22) ; }
    };
}

check_config_build!()