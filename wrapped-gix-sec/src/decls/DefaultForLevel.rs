macro_rules! deps {
    () => {
        Trust!();
    };
}

macro_rules! DefaultForLevel {
    () => {
        deps!();
        # [doc = " A trait to help creating default values based on a trust level."] pub trait DefaultForLevel { # [doc = " Produce a default value for the given trust `level`."] fn default_for_level (level : Trust) -> Self ; }
    };
}

DefaultForLevel!();