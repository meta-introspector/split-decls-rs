// Generated macro for impl_222 (impl)
macro_rules! Depcrate_optionsimpl_222 {
() => {
// Module: crate::options
// Provides: {"impl_222"}
// Dependencies: {}
impl From < CollatorOptions > for CollatorOptionsBitField { fn from (options : CollatorOptions) -> CollatorOptionsBitField { let mut result = Self :: default () ; result . set_strength (options . strength) ; result . set_max_variable (options . max_variable) ; result . set_alternate_handling (options . alternate_handling) ; result . set_case_level_from_enum (options . case_level) ; result } }
};
}
