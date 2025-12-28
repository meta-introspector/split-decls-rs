macro_rules! deps {
    () => {
        Repository!();
        Branch!();
    };
}

macro_rules! worktrees_env_init {
    () => {
        deps!();
        pub fn worktrees_env_init (repo : & Repository) -> (TempDir , Branch < '_ >) { let oid = repo . head () . unwrap () . target () . unwrap () ; let commit = repo . find_commit (oid) . unwrap () ; let branch = repo . branch ("wt-branch" , & commit , true) . unwrap () ; let wtdir = TempDir :: new () . unwrap () ; (wtdir , branch) }
    };
}

worktrees_env_init!();