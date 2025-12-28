macro_rules! git_process {
    () => {
        # [doc = " Run `git $arg_line`, see [`ProcessBuilder`]"] pub fn git_process (arg_line : & str) -> ProcessBuilder { let mut p = process ("git") ; p . arg_line (arg_line) ; p }
    };
}

git_process!();