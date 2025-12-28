macro_rules! UnexpectedCfgCargoMacroHelp {
    () => {
        # [derive (Subdiagnostic)] # [note (lint_unexpected_cfg_from_external_macro_origin)] # [help (lint_unexpected_cfg_from_external_macro_refer)] # [help (lint_unexpected_cfg_cargo_update)] pub (crate) struct UnexpectedCfgCargoMacroHelp { pub macro_kind : & 'static str , pub macro_name : Symbol , pub crate_name : Symbol , }
    };
}

UnexpectedCfgCargoMacroHelp!();