macro_rules! deps {
    () => {
        Command!();
    };
}

macro_rules! xcode_select_developer_dir {
    () => {
        deps!();
        # [doc = " Invoke `xcode-select --print-path`, and return the current developer directory."] # [doc = ""] # [doc = " NOTE: We don't do any error handling here, this is only used as a canary in diagnostics (`xcrun`"] # [doc = " will have already emitted the relevant error information)."] fn xcode_select_developer_dir () -> Option < PathBuf > { let mut cmd = Command :: new ("xcode-select") ; cmd . arg ("--print-path") ; let output = cmd . output () . ok () ? ; if ! output . status . success () { return None ; } Some (stdout_to_path (output . stdout)) }
    };
}

xcode_select_developer_dir!();