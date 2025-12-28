macro_rules! deps {
    () => {
        AnyValueId!();
    };
}

macro_rules! AnyValue {
    () => {
        deps!();
        # [derive (Clone)] pub (crate) struct AnyValue { inner : std :: sync :: Arc < dyn std :: any :: Any + Send + Sync + 'static > , id : AnyValueId , }
    };
}

AnyValue!()