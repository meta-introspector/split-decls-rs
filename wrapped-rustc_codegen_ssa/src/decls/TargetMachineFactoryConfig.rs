macro_rules! TargetMachineFactoryConfig {
    () => {
        # [doc = " Configuration passed to the function returned by the `target_machine_factory`."] pub struct TargetMachineFactoryConfig { # [doc = " Split DWARF is enabled in LLVM by checking that `TM.MCOptions.SplitDwarfFile` isn't empty,"] # [doc = " so the path to the dwarf object has to be provided when we create the target machine."] # [doc = " This can be ignored by backends which do not need it for their Split DWARF support."] pub split_dwarf_file : Option < PathBuf > , # [doc = " The name of the output object file. Used for setting OutputFilenames in target options"] # [doc = " so that LLVM can emit the CodeView S_OBJNAME record in pdb files"] pub output_obj_file : Option < PathBuf > , }
    };
}

TargetMachineFactoryConfig!()