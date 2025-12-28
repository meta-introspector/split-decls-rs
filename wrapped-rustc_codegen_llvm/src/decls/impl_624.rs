macro_rules! deps {
    () => {
        LlvmCodegenBackend!();
    };
}

macro_rules! impl_624 {
    () => {
        deps!();
        impl LlvmCodegenBackend { pub fn new () -> Box < dyn CodegenBackend > { Box :: new (LlvmCodegenBackend (())) } }
    };
}

impl_624!()