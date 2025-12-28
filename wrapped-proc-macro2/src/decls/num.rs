macro_rules! num {
    () => {
        # [cfg (procmacro2_semver_exempt)] mod num ;
    };
}

num!();