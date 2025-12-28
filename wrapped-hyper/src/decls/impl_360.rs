macro_rules! deps {
    () => {
        AsTaskType!();
    };
}

macro_rules! impl_360 {
    () => {
        deps!();
        unsafe impl AsTaskType for () { fn as_task_type (& self) -> hyper_task_return_type { hyper_task_return_type :: HYPER_TASK_EMPTY } }
    };
}

impl_360!();