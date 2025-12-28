macro_rules! TargetLint {
    () => {
        # [doc = " The target of the `by_name` map, which accounts for renaming/deprecation."] # [derive (Debug)] enum TargetLint { # [doc = " A direct lint target"] Id (LintId) , # [doc = " Temporary renaming, used for easing migration pain; see #16545"] Renamed (String , LintId) , # [doc = " Lint with this name existed previously, but has been removed/deprecated."] # [doc = " The string argument is the reason for removal."] Removed (String) , # [doc = " A lint name that should give no warnings and have no effect."] # [doc = ""] # [doc = " This is used by rustc to avoid warning about old rustdoc lints before rustdoc registers"] # [doc = " them as tool lints."] Ignored , }
    };
}

TargetLint!()