macro_rules! AsTaskType {
    () => {
        pub (crate) unsafe trait AsTaskType { fn as_task_type (& self) -> hyper_task_return_type ; }
    };
}

AsTaskType!();