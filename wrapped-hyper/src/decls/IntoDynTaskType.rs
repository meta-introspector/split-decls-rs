macro_rules! deps {
    () => {
        BoxAny!();
    };
}

macro_rules! IntoDynTaskType {
    () => {
        deps!();
        pub (crate) trait IntoDynTaskType { fn into_dyn_task_type (self) -> BoxAny ; }
    };
}

IntoDynTaskType!();