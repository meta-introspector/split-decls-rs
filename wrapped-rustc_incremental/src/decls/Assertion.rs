macro_rules! deps {
    () => {
        Labels!();
    };
}

macro_rules! Assertion {
    () => {
        deps!();
        # [doc = " Represents the requested configuration by rustc_clean/dirty"] struct Assertion { clean : Labels , dirty : Labels , loaded_from_disk : Labels , }
    };
}

Assertion!();