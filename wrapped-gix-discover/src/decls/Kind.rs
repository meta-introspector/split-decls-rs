macro_rules! deps {
    () => {
        Path!();
    };
}

macro_rules! Kind {
    () => {
        deps!();
        # [doc = " The kind of repository path."] # [derive (Debug , Clone , Eq , PartialEq , Ord , PartialOrd , Hash)] pub enum Kind { # [doc = " A bare repository does not have a work tree, that is files on disk beyond the `git` repository itself."] # [doc = ""] # [doc = " Note that this is merely a guess at this point as we didn't read the configuration yet."] # [doc = ""] # [doc = " Also note that due to optimizing for performance and *just* making an educated *guess in some situations*,"] # [doc = " we may consider a non-bare repository bare if it doesn't have an index yet due to be freshly initialized."] # [doc = " The caller has to handle this, typically by reading the configuration."] # [doc = ""] # [doc = " It could also be a directory which is non-bare by configuration, but is *not* named `.git`."] # [doc = " Unusual, but it's possible that a worktree is configured via `core.worktree`."] PossiblyBare , # [doc = " A `git` repository along with checked out files in a work tree."] WorkTree { # [doc = " If set, this is the git dir associated with this _linked_ worktree."] # [doc = " If `None`, the git_dir is the `.git` directory inside the _main_ worktree we represent."] linked_git_dir : Option < PathBuf > , } , # [doc = " A worktree's git directory in the common`.git` directory in `worktrees/<name>`."] WorkTreeGitDir { # [doc = " Path to the worktree directory."] work_dir : PathBuf , } , # [doc = " The directory is a `.git` dir file of a submodule worktree."] Submodule { # [doc = " The git repository itself that is referenced by the `.git` dir file, typically in the `.git/modules/**/<name>` directory of the parent"] # [doc = " repository."] git_dir : PathBuf , } , # [doc = " The git directory in the `.git/modules/**/<name>` directory tree of the parent repository"] SubmoduleGitDir , }
    };
}

Kind!()