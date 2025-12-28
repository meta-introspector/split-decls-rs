macro_rules! deps {
    () => {
        Source!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl Source { # [doc = " Returns true if the source is crates.io."] pub fn is_crates_io (& self) -> bool { self . repr == "registry+https://github.com/rust-lang/crates.io-index" } }
    };
}

impl_61!();