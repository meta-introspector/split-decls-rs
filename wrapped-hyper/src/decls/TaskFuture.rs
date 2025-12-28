macro_rules! TaskFuture {
    () => {
        struct TaskFuture { task : Option < Box < hyper_task > > , }
    };
}

TaskFuture!()