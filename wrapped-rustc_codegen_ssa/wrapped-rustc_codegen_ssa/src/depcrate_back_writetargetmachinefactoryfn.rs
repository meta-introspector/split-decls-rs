// Generated macro for TargetMachineFactoryFn (type)
macro_rules! Depcrate_back_writeTargetMachineFactoryFn {
() => {
// Module: crate::back::write
// Provides: {"TargetMachineFactoryFn"}
// Dependencies: {}
pub type TargetMachineFactoryFn < B > = Arc < dyn Fn (TargetMachineFactoryConfig ,) -> Result < < B as WriteBackendMethods > :: TargetMachine , < B as WriteBackendMethods > :: TargetMachineError , > + Send + Sync , > ;
};
}
