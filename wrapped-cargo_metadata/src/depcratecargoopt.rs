// Generated macro for CargoOpt (enum)
macro_rules! DepcrateCargoOpt {
() => {
// Module: crate
// Provides: {"CargoOpt"}
// Dependencies: {}
# [doc = " Cargo features flags"] # [derive (Debug , Clone)] pub enum CargoOpt { # [doc = " Run cargo with `--features-all`"] AllFeatures , # [doc = " Run cargo with `--no-default-features`"] NoDefaultFeatures , # [doc = " Run cargo with `--features <FEATURES>`"] SomeFeatures (Vec < String >) , }
};
}
