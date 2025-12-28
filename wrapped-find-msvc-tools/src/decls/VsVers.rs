macro_rules! VsVers {
    () => {
        # [doc = " A version of Visual Studio"] # [derive (Debug , PartialEq , Eq , Copy , Clone)] # [non_exhaustive] pub enum VsVers { # [doc = " Visual Studio 12 (2013)"] # [deprecated (note = "Visual Studio 12 is no longer supported. cc will never return this value.")] Vs12 , # [doc = " Visual Studio 14 (2015)"] Vs14 , # [doc = " Visual Studio 15 (2017)"] Vs15 , # [doc = " Visual Studio 16 (2019)"] Vs16 , # [doc = " Visual Studio 17 (2022)"] Vs17 , # [doc = " Visual Studio 18 (2026)"] Vs18 , }
    };
}

VsVers!();