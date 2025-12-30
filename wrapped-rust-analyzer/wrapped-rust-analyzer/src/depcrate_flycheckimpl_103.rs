// Generated macro for impl_103 (impl)
macro_rules! Depcrate_flycheckimpl_103 {
() => {
// Module: crate::flycheck
// Provides: {"impl_103"}
// Dependencies: {}
impl CargoOptions { pub (crate) fn apply_on_command (& self , cmd : & mut Command) { for target in & self . target_tuples { cmd . args (["--target" , target . as_str ()]) ; } if self . all_targets { if self . set_test { cmd . arg ("--all-targets") ; } else { cmd . args (["--lib" , "--bins" , "--examples"]) ; } } if self . all_features { cmd . arg ("--all-features") ; } else { if self . no_default_features { cmd . arg ("--no-default-features") ; } if ! self . features . is_empty () { cmd . arg ("--features") ; cmd . arg (self . features . join (" ")) ; } } if let Some (target_dir) = & self . target_dir { cmd . arg ("--target-dir") . arg (target_dir) ; } } }
};
}
