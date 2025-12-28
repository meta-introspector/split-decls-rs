macro_rules! SysrootPublicDeps {
    () => {
        # [derive (Default , Debug)] struct SysrootPublicDeps { deps : Vec < (CrateName , CrateBuilderId , bool) > , }
    };
}

SysrootPublicDeps!();