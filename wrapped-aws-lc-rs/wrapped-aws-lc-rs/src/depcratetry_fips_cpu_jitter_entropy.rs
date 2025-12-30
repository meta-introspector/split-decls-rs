// Generated macro for try_fips_cpu_jitter_entropy (function)
macro_rules! Depcratetry_fips_cpu_jitter_entropy {
() => {
// Module: crate
// Provides: {"try_fips_cpu_jitter_entropy"}
// Dependencies: {}
# [doc = " Indicates whether the underlying implementation is FIPS."] # [doc = ""] # [doc = " # Errors"] # [doc = " Return an error if the underlying implementation is not using CPU jitter entropy, otherwise Ok."] pub fn try_fips_cpu_jitter_entropy () -> Result < () , & 'static str > { init () ; # [cfg (feature = "fips")] if aws_lc :: CFG_CPU_JITTER_ENTROPY () { Ok (()) } else { Err ("FIPS CPU Jitter Entropy not enabled!") } # [cfg (not (feature = "fips"))] match unsafe { aws_lc :: FIPS_is_entropy_cpu_jitter () } { 1 => Ok (()) , _ => Err ("FIPS CPU Jitter Entropy not enabled!") , } }
};
}
