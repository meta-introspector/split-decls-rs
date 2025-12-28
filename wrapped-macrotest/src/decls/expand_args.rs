macro_rules! deps {
    () => {
        ExpansionBehavior!();
    };
}

macro_rules! expand_args {
    () => {
        deps!();
        # [doc = " Same as [`expand`] but allows to pass additional arguments to `cargo-expand`."] # [doc = ""] # [doc = " [`expand`]: expand/fn.expand.html"] pub fn expand_args < I , S > (path : impl AsRef < Path > , args : I) where I : IntoIterator < Item = S > + Clone , S : AsRef < OsStr > , { run_tests (path , ExpansionBehavior :: RegenerateFiles , Some (args)) ; }
    };
}

expand_args!();