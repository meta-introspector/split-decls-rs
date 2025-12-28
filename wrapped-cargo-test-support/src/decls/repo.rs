macro_rules! deps {
    () => {
        RepoBuilder!();
    };
}

macro_rules! repo {
    () => {
        deps!();
        # [doc = " Create a [`RepoBuilder`] to build a new git repository."] # [doc = ""] # [doc = " Call [`RepoBuilder::build()`] to finalize and create the repository."] pub fn repo (p : & Path) -> RepoBuilder { RepoBuilder :: init (p) }
    };
}

repo!()