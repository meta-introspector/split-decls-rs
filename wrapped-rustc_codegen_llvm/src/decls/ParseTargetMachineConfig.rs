macro_rules! deps {
    () => {
        LlvmError!();
    };
}

macro_rules! ParseTargetMachineConfig {
    () => {
        deps!();
        pub (crate) struct ParseTargetMachineConfig < 'a > (pub LlvmError < 'a >) ;
    };
}

ParseTargetMachineConfig!();