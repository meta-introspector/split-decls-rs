macro_rules! DebugCuIndex {
    () => {
        # [doc = " The data in the `.debug_cu_index` section of a `.dwp` file."] # [doc = ""] # [doc = " This section contains the compilation unit index."] # [derive (Debug , Default , Clone , Copy)] pub struct DebugCuIndex < R > { section : R , }
    };
}

DebugCuIndex!()