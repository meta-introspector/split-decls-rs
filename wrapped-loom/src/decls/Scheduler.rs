macro_rules! Scheduler {
    () => {
        pub (crate) struct Scheduler { max_threads : usize , }
    };
}

Scheduler!()