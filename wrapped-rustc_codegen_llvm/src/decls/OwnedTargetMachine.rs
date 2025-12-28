macro_rules! OwnedTargetMachine {
    () => {
        # [doc = " Responsible for safely creating and disposing llvm::TargetMachine via ffi functions."] # [doc = " Not cloneable as there is no clone function for llvm::TargetMachine."] # [repr (transparent)] pub struct OwnedTargetMachine { tm_unique : NonNull < llvm :: TargetMachine > , phantom : PhantomData < llvm :: TargetMachine > , }
    };
}

OwnedTargetMachine!();