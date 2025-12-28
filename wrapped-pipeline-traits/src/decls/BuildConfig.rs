macro_rules! BuildConfig {
    () => {
        # [derive (Debug , Deserialize , Clone)] pub struct BuildConfig { pub stage : Option < String > , pub target : Option < String > , # [serde (rename = "patch-binaries-for-nix")] pub patch_binaries_for_nix : Option < bool > , pub vendor : Option < bool > , # [serde (rename = "build-dir")] pub build_dir : Option < String > , pub jobs : Option < usize > , }
    };
}

BuildConfig!()