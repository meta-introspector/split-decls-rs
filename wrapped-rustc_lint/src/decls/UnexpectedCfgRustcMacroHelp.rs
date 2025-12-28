macro_rules! UnexpectedCfgRustcMacroHelp {
    () => {
        # [derive (Subdiagnostic)] # [note (lint_unexpected_cfg_from_external_macro_origin)] # [help (lint_unexpected_cfg_from_external_macro_refer)] pub (crate) struct UnexpectedCfgRustcMacroHelp { pub macro_kind : & 'static str , pub macro_name : Symbol , }
    };
}

UnexpectedCfgRustcMacroHelp!()