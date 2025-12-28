macro_rules! deps {
    () => {
        TestEnvCommandExt!();
    };
}

macro_rules! process {
    () => {
        deps!();
        # [doc = " Run `$bin` in the test's environment, see [`ProcessBuilder`]"] # [doc = ""] # [doc = " For more on the test environment, see"] # [doc = " - [`paths::root`]"] # [doc = " - [`TestEnvCommandExt`]"] pub fn process < T : AsRef < OsStr > > (bin : T) -> ProcessBuilder { _process (bin . as_ref ()) }
    };
}

process!()