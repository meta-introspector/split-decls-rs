macro_rules! deps {
    () => {
        ParamHint!();
    };
}

macro_rules! impl_279 {
    () => {
        deps!();
        impl ParamHint { fn is_array (& self) -> bool { matches ! (self , Self :: ArrayFixed (_) | Self :: ArrayRelativeLen (_) | Self :: ArrayRelativeByteLen (_) | Self :: ArrayRelativePtr (_)) } }
    };
}

impl_279!()