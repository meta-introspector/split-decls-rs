macro_rules! deps {
    () => {
        Error!();
        AsTaskType!();
    };
}

macro_rules! impl_361 {
    () => {
        deps!();
        unsafe impl AsTaskType for crate :: Error { fn as_task_type (& self) -> hyper_task_return_type { hyper_task_return_type :: HYPER_TASK_ERROR } }
    };
}

impl_361!()