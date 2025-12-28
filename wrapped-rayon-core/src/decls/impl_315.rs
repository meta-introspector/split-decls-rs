macro_rules! deps {
    () => {
        ThreadPoolBuilder!();
        DefaultSpawn!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        impl Default for ThreadPoolBuilder { fn default () -> Self { ThreadPoolBuilder { num_threads : 0 , use_current_thread : false , panic_handler : None , get_thread_name : None , stack_size : None , start_handler : None , exit_handler : None , spawn_handler : DefaultSpawn , breadth_first : false , } } }
    };
}

impl_315!()