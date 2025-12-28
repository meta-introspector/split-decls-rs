macro_rules! TomlPackageBuild {
    () => {
        # [derive (Clone , Debug , Serialize , Eq , PartialEq)] # [serde (untagged)] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub enum TomlPackageBuild { # [doc = " If build scripts are disabled or enabled."] # [doc = " If true, `build.rs` in the root folder will be the build script."] Auto (bool) , # [doc = " Path of Build Script if there's just one script."] SingleScript (String) , # [doc = " Vector of paths if multiple build script are to be used."] MultipleScript (Vec < String >) , }
    };
}

TomlPackageBuild!();