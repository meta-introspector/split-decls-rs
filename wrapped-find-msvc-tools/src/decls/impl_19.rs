macro_rules! deps {
    () => {
        Tool!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl Tool { # [doc = " Converts this compiler into a `Command` that's ready to be run."] # [doc = ""] # [doc = " This is useful for when the compiler needs to be executed and the"] # [doc = " command returned will already have the initial arguments and environment"] # [doc = " variables configured."] pub fn to_command (& self) -> Command { let mut cmd = Command :: new (& self . tool) ; for (k , v) in self . env . iter () { cmd . env (k , v) ; } cmd } # [doc = " Check is the tool clang-cl related"] pub fn is_clang_cl (& self) -> bool { self . is_clang_cl } # [doc = " Get path to the tool"] pub fn path (& self) -> & Path { & self . tool } # [doc = " Get environment variables for the tools"] pub fn env (& self) -> impl IntoIterator < Item = & (OsString , OsString) > { & self . env } }
    };
}

impl_19!();