macro_rules! deps {
    () => {
        ManifestPath!();
        PackageData!();
        TargetData!();
    };
}

macro_rules! CargoWorkspace {
    () => {
        deps!();
        # [doc = " [`CargoWorkspace`] represents the logical structure of, well, a Cargo"] # [doc = " workspace. It pretty closely mirrors `cargo metadata` output."] # [doc = ""] # [doc = " Note that internally, rust-analyzer uses a different structure:"] # [doc = " `CrateGraph`. `CrateGraph` is lower-level: it knows only about the crates,"] # [doc = " while this knows about `Packages` & `Targets`: purely cargo-related"] # [doc = " concepts."] # [doc = ""] # [doc = " We use absolute paths here, `cargo metadata` guarantees to always produce"] # [doc = " abs paths."] # [derive (Debug , Clone , Eq , PartialEq)] pub struct CargoWorkspace { packages : Arena < PackageData > , targets : Arena < TargetData > , workspace_root : AbsPathBuf , target_directory : AbsPathBuf , manifest_path : ManifestPath , is_virtual_workspace : bool , # [doc = " Whether this workspace represents the sysroot workspace."] is_sysroot : bool , # [doc = " Environment variables set in the `.cargo/config` file and the extraEnv"] # [doc = " configuration option."] env : Env , requires_rustc_private : bool , }
    };
}

CargoWorkspace!();