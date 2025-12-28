macro_rules! deps {
    () => {
        CfgOverrides!();
        Sysroot!();
        ProjectWorkspaceKind!();
    };
}

macro_rules! ProjectWorkspace {
    () => {
        deps!();
        # [derive (Clone)] pub struct ProjectWorkspace { pub kind : ProjectWorkspaceKind , # [doc = " The sysroot loaded for this workspace."] pub sysroot : Sysroot , # [doc = " Holds cfg flags for the current target. We get those by running"] # [doc = " `rustc --print cfg`."] pub rustc_cfg : Vec < CfgAtom > , # [doc = " The toolchain version used by this workspace."] pub toolchain : Option < Version > , # [doc = " The target data layout queried for workspace."] pub target : TargetLoadResult , # [doc = " A set of cfg overrides for this workspace."] pub cfg_overrides : CfgOverrides , # [doc = " Additional includes to add for the VFS."] pub extra_includes : Vec < AbsPathBuf > , # [doc = " Set `cfg(test)` for local crates"] pub set_test : bool , }
    };
}

ProjectWorkspace!()