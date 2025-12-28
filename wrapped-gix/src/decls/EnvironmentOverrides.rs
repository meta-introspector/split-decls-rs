macro_rules! deps {
    () => {
        Clone!();
        Default!();
    };
}

macro_rules! EnvironmentOverrides {
    () => {
        deps!();
        # [derive (Default , Clone)] pub (crate) struct EnvironmentOverrides { # [doc = " An override of the worktree typically from the environment, and overrides even worktree dirs set as parameter."] # [doc = ""] # [doc = " This emulates the way git handles this override."] worktree_dir : Option < PathBuf > , # [doc = " An override for the .git directory, typically from the environment."] # [doc = ""] # [doc = " If set, the passed in `git_dir` parameter will be ignored in favor of this one."] git_dir : Option < PathBuf > , }
    };
}

EnvironmentOverrides!()