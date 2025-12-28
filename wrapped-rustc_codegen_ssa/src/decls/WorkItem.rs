macro_rules! deps {
    () => {
        ModuleCodegen!();
        CachedModuleCodegen!();
        SerializedModule!();
        WriteBackendMethods!();
        FatLtoInput!();
        ThinModule!();
    };
}

macro_rules! WorkItem {
    () => {
        deps!();
        pub (crate) enum WorkItem < B : WriteBackendMethods > { # [doc = " Optimize a newly codegened, totally unoptimized module."] Optimize (ModuleCodegen < B :: Module >) , # [doc = " Copy the post-LTO artifacts from the incremental cache to the output"] # [doc = " directory."] CopyPostLtoArtifacts (CachedModuleCodegen) , # [doc = " Performs fat LTO on the given module."] FatLto { exported_symbols_for_lto : Arc < Vec < String > > , each_linked_rlib_for_lto : Vec < PathBuf > , needs_fat_lto : Vec < FatLtoInput < B > > , import_only_modules : Vec < (SerializedModule < B :: ModuleBuffer > , WorkProduct) > , } , # [doc = " Performs thin-LTO on the given module."] ThinLto (lto :: ThinModule < B >) , }
    };
}

WorkItem!();