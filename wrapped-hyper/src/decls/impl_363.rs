macro_rules! deps {
    () => {
        BoxAny!();
        IntoDynTaskType!();
        Result!();
    };
}

macro_rules! impl_363 {
    () => {
        deps!();
        impl < T > IntoDynTaskType for crate :: Result < T > where T : IntoDynTaskType + Send + Sync + 'static , { fn into_dyn_task_type (self) -> BoxAny { match self { Ok (val) => val . into_dyn_task_type () , Err (err) => Box :: new (err) , } } }
    };
}

impl_363!()