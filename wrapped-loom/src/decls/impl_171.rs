macro_rules! deps {
    () => {
        LocalValue!();
        AccessError!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl LocalValue { fn new < T : 'static > (value : T) -> Self { Self (Some (Box :: new (value))) } fn get < T : 'static > (& self) -> Result < & T , AccessError > { self . 0 . as_ref () . ok_or (AccessError { _private : () }) . map (| val | { val . downcast_ref :: < T > () . expect ("local value must downcast to expected type") }) } }
    };
}

impl_171!()