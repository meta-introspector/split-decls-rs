macro_rules! deps {
    () => {
        IntoDynTaskType!();
        BoxAny!();
        AsTaskType!();
    };
}

macro_rules! impl_362 {
    () => {
        deps!();
        impl < T > IntoDynTaskType for T where T : AsTaskType + Send + Sync + 'static , { fn into_dyn_task_type (self) -> BoxAny { Box :: new (self) } }
    };
}

impl_362!();