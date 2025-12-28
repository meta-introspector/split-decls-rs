macro_rules! deps {
    () => {
        UnexpectedCfgCargoHelp!();
    };
}

macro_rules! impl_560 {
    () => {
        deps!();
        impl UnexpectedCfgCargoHelp { fn cargo_toml_lint_cfg (unescaped : & str) -> String { format ! ("\n [lints.rust]\n unexpected_cfgs = {{ level = \"warn\", check-cfg = ['{unescaped}'] }}") } pub (crate) fn lint_cfg (unescaped : & str) -> Self { UnexpectedCfgCargoHelp :: LintCfg { cargo_toml_lint_cfg : Self :: cargo_toml_lint_cfg (unescaped) , } } pub (crate) fn lint_cfg_and_build_rs (unescaped : & str , escaped : & str) -> Self { UnexpectedCfgCargoHelp :: LintCfgAndBuildRs { cargo_toml_lint_cfg : Self :: cargo_toml_lint_cfg (unescaped) , build_rs_println : format ! ("println!(\"cargo::rustc-check-cfg={escaped}\");") , } } }
    };
}

impl_560!()