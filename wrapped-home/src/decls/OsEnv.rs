macro_rules! deps {
    () => {
        Env!();
    };
}

macro_rules! OsEnv {
    () => {
        deps!();
        # [doc = " Implements Env for the OS context, both Unix style and Windows."] # [doc = ""] # [doc = " This is trait permits in-process testing by providing a control point to"] # [doc = " allow in-process divergence on what is normally process wide state."] # [doc = ""] # [doc = " Implementations should be provided by whatever testing framework the caller"] # [doc = " is using. Code that is not performing in-process threaded testing requiring"] # [doc = " isolated rustup/cargo directories does not need this trait or the _from"] # [doc = " functions."] pub struct OsEnv ;
    };
}

OsEnv!()