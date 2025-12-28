macro_rules! deps {
    () => {
        DisplaySourceCodeError!();
    };
}

macro_rules! HirDisplayError {
    () => {
        deps!();
        pub enum HirDisplayError { # [doc = " Errors that can occur when generating source code"] DisplaySourceCodeError (DisplaySourceCodeError) , # [doc = " `FmtError` is required to be compatible with std::fmt::Display"] FmtError , }
    };
}

HirDisplayError!()