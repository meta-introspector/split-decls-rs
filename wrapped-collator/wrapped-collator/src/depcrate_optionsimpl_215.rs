// Generated macro for impl_215 (impl)
macro_rules! Depcrate_optionsimpl_215 {
() => {
// Module: crate::options
// Provides: {"impl_215"}
// Dependencies: {}
impl From < ResolvedCollatorOptions > for CollatorOptions { # [doc = " Convenience conversion for copying the options from an"] # [doc = " existing collator into a new one (overriding any locale-provided"] # [doc = " defaults of the new one!)."] fn from (options : ResolvedCollatorOptions) -> CollatorOptions { Self { strength : Some (options . strength) , alternate_handling : Some (options . alternate_handling) , max_variable : Some (options . max_variable) , case_level : Some (options . case_level) , } } }
};
}
