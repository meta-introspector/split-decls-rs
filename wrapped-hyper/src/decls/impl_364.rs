macro_rules! deps {
    () => {
        BoxAny!();
        IntoDynTaskType!();
    };
}

macro_rules! impl_364 {
    () => {
        deps!();
        impl < T > IntoDynTaskType for Option < T > where T : IntoDynTaskType + Send + Sync + 'static , { fn into_dyn_task_type (self) -> BoxAny { match self { Some (val) => val . into_dyn_task_type () , None => () . into_dyn_task_type () , } } }
    };
}

impl_364!();