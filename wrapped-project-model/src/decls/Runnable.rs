macro_rules! deps {
    () => {
        RunnableKind!();
    };
}

macro_rules! Runnable {
    () => {
        deps!();
        # [doc = " A template-like structure for describing runnables."] # [doc = ""] # [doc = " These are used for running and debugging binaries and tests without encoding"] # [doc = " build system-specific knowledge into rust-analyzer."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " Below is an example of a test runnable. `{label}` and `{test_id}`"] # [doc = " are explained in [`Runnable::args`]'s documentation."] # [doc = ""] # [doc = " ```json"] # [doc = " {"] # [doc = "     \"program\": \"buck\","] # [doc = "     \"args\": ["] # [doc = "         \"test\","] # [doc = "          \"{label}\","] # [doc = "          \"--\","] # [doc = "          \"{test_id}\","] # [doc = "          \"--print-passing-details\""] # [doc = "     ],"] # [doc = "     \"cwd\": \"/home/user/repo-root/\","] # [doc = "     \"kind\": \"testOne\""] # [doc = " }"] # [doc = " ```"] # [derive (Debug , Clone , PartialEq , Eq)] pub struct Runnable { # [doc = " The program invoked by the runnable."] # [doc = ""] # [doc = " For example, this might be `cargo`, `buck`, or `bazel`."] pub program : String , # [doc = " The arguments passed to [`Runnable::program`]."] # [doc = ""] # [doc = " The args can contain two template strings: `{label}` and `{test_id}`."] # [doc = " rust-analyzer will find and replace `{label}` with [`Build::label`] and"] # [doc = " `{test_id}` with the test name."] pub args : Vec < String > , # [doc = " The current working directory of the runnable."] pub cwd : Utf8PathBuf , pub kind : RunnableKind , }
    };
}

Runnable!();