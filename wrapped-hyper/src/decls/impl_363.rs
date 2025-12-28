macro_rules! deps {
    () => {
        BoxAny!();
        Result!();
        IntoDynTaskType!();
    };
}

macro_rules! impl_363 {
    () => {
        deps!();
        impl < T > IntoDynTaskType for crate :: Result < T > where T : IntoDynTaskType + Send + Sync + 'static , { fn into_dyn_task_type (self) -> BoxAny { match self { Ok (val) => val . into_dyn_task_type () , Err (err) => Box :: new (err) , } } }
    };
}

impl_363!();