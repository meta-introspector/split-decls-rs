macro_rules! deps {
    () => {
        LlvmError!();
    };
}

macro_rules! WithLlvmError {
    () => {
        deps!();
        pub (crate) struct WithLlvmError < 'a > (pub LlvmError < 'a > , pub String) ;
    };
}

WithLlvmError!()