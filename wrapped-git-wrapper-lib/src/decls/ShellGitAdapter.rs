macro_rules! deps {
    () => {
        Execv!();
        GitAdapter!();
    };
}

macro_rules! ShellGitAdapter {
    () => {
        deps!();
        # [doc = " Shell implementation of `GitAdapter` that uses external `git` commands."] pub struct ShellGitAdapter { execv : Arc < dyn Execv + Send + Sync > , }
    };
}

ShellGitAdapter!()