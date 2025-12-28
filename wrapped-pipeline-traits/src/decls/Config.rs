macro_rules! deps {
    () => {
        DistConfig!();
        LlvmConfig!();
        BuildConfig!();
        EnvConfig!();
        NixConfig!();
        RustConfig!();
        ChangeIdConfig!();
        InstallConfig!();
        ModuleExportsConfig!();
        BinsConfig!();
    };
}

macro_rules! Config {
    () => {
        deps!();
        # [derive (Debug , Deserialize , Clone)] pub struct Config { # [serde (default)] pub nix : Option < NixConfig > , # [serde (default)] pub rust : Option < RustConfig > , # [serde (default)] pub build : Option < BuildConfig > , # [serde (default)] pub env : Option < EnvConfig > , # [serde (default)] pub install : Option < InstallConfig > , # [serde (default)] pub dist : Option < DistConfig > , # [serde (default)] pub llvm : Option < LlvmConfig > , # [serde (default , rename = "change-id")] pub change_id : Option < ChangeIdConfig > , # [serde (default)] pub bins : Option < BinsConfig > , # [serde (default , rename = "module_exports")] pub module_exports : Option < ModuleExportsConfig > , # [serde (default)] pub generated_output_dir : Option < PathBuf > , }
    };
}

Config!()