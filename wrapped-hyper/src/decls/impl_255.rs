macro_rules! deps {
    () => {
        AsTaskType!();
    };
}

macro_rules! impl_255 {
    () => {
        deps!();
        unsafe impl AsTaskType for hyper_buf { fn as_task_type (& self) -> hyper_task_return_type { hyper_task_return_type :: HYPER_TASK_BUF } }
    };
}

impl_255!()