// Generated macro for hijri (module)
macro_rules! Depcrate_calhijri {
() => {
// Module: crate::cal
// Provides: {"hijri"}
// Dependencies: {}
# [doc = " Customizations for the [`Hijri`] calendar."] pub mod hijri { pub use super :: hijri_internal :: { AstronomicalSimulation , TabularAlgorithm , TabularAlgorithmEpoch , TabularAlgorithmLeapYears , UmmAlQura , } ; # [cfg (feature = "unstable")] pub use super :: hijri_internal :: { HijriYear , Rules } ; # [doc (hidden)] # [doc = " These are unstable traits but we expose them on stable to"] # [doc = " icu_datetime."] pub mod unstable_internal { pub use super :: super :: hijri_internal :: Rules ; } }
};
}
