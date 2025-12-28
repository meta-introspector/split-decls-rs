macro_rules! RealWorkspaceRemover {
    () => {
        # [cfg (not (feature = "cargo-toml-editor-lib"))] pub struct RealWorkspaceRemover ;
    };
}

RealWorkspaceRemover!();