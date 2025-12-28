macro_rules! deps {
    () => {
        AsTaskType!();
    };
}

macro_rules! impl_302 {
    () => {
        deps!();
        unsafe impl AsTaskType for hyper_response { fn as_task_type (& self) -> hyper_task_return_type { hyper_task_return_type :: HYPER_TASK_RESPONSE } }
    };
}

impl_302!();