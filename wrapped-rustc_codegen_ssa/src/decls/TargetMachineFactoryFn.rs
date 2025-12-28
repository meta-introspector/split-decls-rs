macro_rules! deps {
    () => {
        WriteBackendMethods!();
        TargetMachineFactoryConfig!();
    };
}

macro_rules! TargetMachineFactoryFn {
    () => {
        deps!();
        pub type TargetMachineFactoryFn < B > = Arc < dyn Fn (TargetMachineFactoryConfig ,) -> Result < < B as WriteBackendMethods > :: TargetMachine , < B as WriteBackendMethods > :: TargetMachineError , > + Send + Sync , > ;
    };
}

TargetMachineFactoryFn!();