macro_rules! deps {
    () => {
        AnyValueId!();
    };
}

macro_rules! impl_603 {
    () => {
        deps!();
        impl AnyValueId { pub (crate) fn of < A : ? Sized + 'static > () -> Self { Self { type_id : std :: any :: TypeId :: of :: < A > () , # [cfg (debug_assertions)] type_name : std :: any :: type_name :: < A > () , } } }
    };
}

impl_603!();