macro_rules! deps {
    () => {
        AsTaskType!();
    };
}

macro_rules! impl_263 {
    () => {
        deps!();
        unsafe impl AsTaskType for hyper_clientconn { fn as_task_type (& self) -> hyper_task_return_type { hyper_task_return_type :: HYPER_TASK_CLIENTCONN } }
    };
}

impl_263!()