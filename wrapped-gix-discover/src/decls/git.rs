macro_rules! deps {
    () => {
        Error!();
        Kind!();
        Path!();
    };
}

macro_rules! git {
    () => {
        deps!();
        # [doc = " What constitutes a valid git repository, returning the guessed repository kind"] # [doc = " purely based on the presence of files. Note that the git-config ultimately decides what's bare."] # [doc = ""] # [doc = " Returns the `Kind` of git directory that was passed, possibly alongside the supporting private worktree git dir."] # [doc = ""] # [doc = " Note that `.git` files are followed to a valid git directory, which then requires…"] # [doc = ""] # [doc = "   * …a valid head"] # [doc = "   * …an objects directory"] # [doc = "   * …a refs directory"] # [doc = ""] pub fn git (git_dir : & Path) -> Result < crate :: repository :: Kind , crate :: is_git :: Error > { let git_dir_metadata = git_dir . metadata () . map_err (| err | crate :: is_git :: Error :: Metadata { source : err , path : git_dir . into () , }) ? ; let cwd = gix_fs :: current_dir (false) ? ; git_with_metadata (git_dir , git_dir_metadata , & cwd) }
    };
}

git!();