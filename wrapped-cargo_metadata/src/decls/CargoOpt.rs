macro_rules! CargoOpt {
    () => {
        # [doc = " Cargo features flags"] # [derive (Debug , Clone)] pub enum CargoOpt { # [doc = " Run cargo with `--features-all`"] AllFeatures , # [doc = " Run cargo with `--no-default-features`"] NoDefaultFeatures , # [doc = " Run cargo with `--features <FEATURES>`"] SomeFeatures (Vec < String >) , }
    };
}

CargoOpt!();