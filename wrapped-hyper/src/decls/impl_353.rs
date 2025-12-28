macro_rules! deps {
    () => {
        IntoDynTaskType!();
        UserDataPointer!();
    };
}

macro_rules! impl_353 {
    () => {
        deps!();
        impl hyper_task { pub (crate) fn boxed < F > (fut : F) -> Box < hyper_task > where F : Future + Send + 'static , F :: Output : IntoDynTaskType + Send + Sync + 'static , { Box :: new (hyper_task { future : Box :: pin (async move { fut . await . into_dyn_task_type () }) , output : None , userdata : UserDataPointer (ptr :: null_mut ()) , }) } fn output_type (& self) -> hyper_task_return_type { match self . output { None => hyper_task_return_type :: HYPER_TASK_EMPTY , Some (ref val) => val . as_task_type () , } } }
    };
}

impl_353!();