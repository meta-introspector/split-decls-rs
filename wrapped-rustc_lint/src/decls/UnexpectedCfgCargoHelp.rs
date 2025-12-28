macro_rules! UnexpectedCfgCargoHelp {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum UnexpectedCfgCargoHelp { # [help (lint_unexpected_cfg_add_cargo_feature)] # [help (lint_unexpected_cfg_add_cargo_toml_lint_cfg)] LintCfg { cargo_toml_lint_cfg : String } , # [help (lint_unexpected_cfg_add_cargo_feature)] # [help (lint_unexpected_cfg_add_cargo_toml_lint_cfg)] # [help (lint_unexpected_cfg_add_build_rs_println)] LintCfgAndBuildRs { cargo_toml_lint_cfg : String , build_rs_println : String } , }
    };
}

UnexpectedCfgCargoHelp!()