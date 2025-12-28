macro_rules! deps {
    () => {
        Properties!();
    };
}

macro_rules! Default {
    () => {
        deps!();
        # [doc = " Not public version of `std::default::Default`, used to not leak default constructors into the"] # [doc = " public API"] trait Default { # [doc = " Creates `Properties` with default configuration"] fn default () -> Self ; }
    };
}

Default!();