// Generated macro for OwnedTargetMachine (struct)
macro_rules! Depcrate_back_owned_target_machineOwnedTargetMachine {
() => {
// Module: crate::back::owned_target_machine
// Provides: {"OwnedTargetMachine"}
// Dependencies: {}
# [doc = " Responsible for safely creating and disposing llvm::TargetMachine via ffi functions."] # [doc = " Not cloneable as there is no clone function for llvm::TargetMachine."] # [repr (transparent)] pub struct OwnedTargetMachine { tm_unique : NonNull < llvm :: TargetMachine > , phantom : PhantomData < llvm :: TargetMachine > , }
};
}
