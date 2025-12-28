macro_rules! ModuleLlvm {
    () => {
        pub struct ModuleLlvm { llcx : & 'static mut llvm :: Context , llmod_raw : * const llvm :: Module , tm : ManuallyDrop < OwnedTargetMachine > , }
    };
}

ModuleLlvm!()