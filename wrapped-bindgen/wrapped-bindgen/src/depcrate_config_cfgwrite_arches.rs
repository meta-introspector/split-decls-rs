// Generated macro for write_arches (function)
macro_rules! Depcrate_config_cfgwrite_arches {
() => {
// Module: crate::config::cfg
// Provides: {"write_arches"}
// Dependencies: {}
pub fn write_arches < R : HasAttributes > (row : R) -> TokenStream { let mut tokens = quote ! { } ; if let Some (attribute) = row . find_attribute ("SupportedArchitectureAttribute") { if let Some ((_ , Value :: I32 (value))) = attribute . args () . first () { let mut arches = BTreeSet :: new () ; if value & 1 == 1 { arches . insert ("x86") ; } if value & 2 == 2 { arches . insert ("x86_64") ; arches . insert ("arm64ec") ; } if value & 4 == 4 { arches . insert ("aarch64") ; } match arches . len () { 0 => { } 1 => tokens . combine (quote ! { # [cfg (# (target_arch = # arches) ,*)] }) , _ => tokens . combine (quote ! { # [cfg (any (# (target_arch = # arches) ,*))] }) , } } } tokens }
};
}
