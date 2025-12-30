// Generated macro for impl_109 (impl)
macro_rules! Depcrate_flycheckimpl_109 {
() => {
// Module: crate::flycheck
// Provides: {"impl_109"}
// Dependencies: {}
impl CargoOptions { pub (crate) fn apply_on_command (& self , cmd : & mut Command , ws_target_dir : Option < & Utf8Path >) { for target in & self . target_tuples { cmd . args (["--target" , target . as_str ()]) ; } if self . all_targets { if self . set_test { cmd . arg ("--all-targets") ; } else { cmd . args (["--lib" , "--bins" , "--examples"]) ; } } if self . all_features { cmd . arg ("--all-features") ; } else { if self . no_default_features { cmd . arg ("--no-default-features") ; } if ! self . features . is_empty () { cmd . arg ("--features") ; cmd . arg (self . features . join (" ")) ; } } if let Some (target_dir) = self . target_dir_config . target_dir (ws_target_dir) { cmd . arg ("--target-dir") . arg (target_dir . as_ref ()) ; } } }
};
}
