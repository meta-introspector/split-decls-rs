macro_rules! ModuleFlagMergeBehavior {
    () => {
        # [doc = " Must match the layout of `LLVMRustModuleFlagMergeBehavior`."] # [doc = ""] # [doc = " When merging modules (e.g. during LTO), their metadata flags are combined. Conflicts are"] # [doc = " resolved according to the merge behaviors specified here. Flags differing only in merge"] # [doc = " behavior are still considered to be in conflict."] # [doc = ""] # [doc = " In order for Rust-C LTO to work, we must specify behaviors compatible with Clang. Notably,"] # [doc = " 'Error' and 'Warning' cannot be mixed for a given flag."] # [doc = ""] # [doc = " There is a stable LLVM-C version of this enum (`LLVMModuleFlagBehavior`),"] # [doc = " but as of LLVM 19 it does not support all of the enum values in the unstable"] # [doc = " C++ API."] # [derive (Copy , Clone , PartialEq)] # [repr (C)] pub (crate) enum ModuleFlagMergeBehavior { Error = 1 , Warning = 2 , Require = 3 , Override = 4 , Append = 5 , AppendUnique = 6 , Max = 7 , Min = 8 , }
    };
}

ModuleFlagMergeBehavior!();