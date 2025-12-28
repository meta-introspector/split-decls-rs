macro_rules! deps {
    () => {
        OwnedTargetMachine!();
    };
}

macro_rules! ModuleLlvm {
    () => {
        deps!();
        pub struct ModuleLlvm { llcx : & 'static mut llvm :: Context , llmod_raw : * const llvm :: Module , tm : ManuallyDrop < OwnedTargetMachine > , }
    };
}

ModuleLlvm!();