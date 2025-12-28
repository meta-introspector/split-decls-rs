macro_rules! UnexpectedCfgRustcHelp {
    () => {
        # [derive (Subdiagnostic)] # [help (lint_unexpected_cfg_add_cmdline_arg)] pub (crate) struct UnexpectedCfgRustcHelp { pub cmdline_arg : String , }
    };
}

UnexpectedCfgRustcHelp!()