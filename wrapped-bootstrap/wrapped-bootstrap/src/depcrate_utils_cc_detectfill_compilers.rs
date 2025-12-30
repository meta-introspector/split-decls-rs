// Generated macro for fill_compilers (function)
macro_rules! Depcrate_utils_cc_detectfill_compilers {
() => {
// Module: crate::utils::cc_detect
// Provides: {"fill_compilers"}
// Dependencies: {}
# [doc = " Probes for C and C++ compilers and configures the corresponding entries in the [`Build`]"] # [doc = " structure."] # [doc = ""] # [doc = " This function determines which targets need a C compiler (and, if needed, a C++ compiler)"] # [doc = " by combining the primary build target, host targets, and any additional targets. For"] # [doc = " each target, it calls [`fill_target_compiler`] to configure the necessary compiler tools."] pub fn fill_compilers (build : & mut Build) { let targets : HashSet < _ > = match build . config . cmd { crate :: Subcommand :: Clean { .. } | crate :: Subcommand :: Check { .. } | crate :: Subcommand :: Format { .. } | crate :: Subcommand :: Setup { .. } => { build . hosts . iter () . cloned () . chain (iter :: once (build . host_target)) . collect () } _ => { build . targets . iter () . chain (& build . hosts) . cloned () . chain (iter :: once (build . host_target)) . collect () } } ; for target in targets . into_iter () { fill_target_compiler (build , target) ; } }
};
}
