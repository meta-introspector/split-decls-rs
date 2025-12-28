macro_rules! deps {
    () => {
        Repository!();
    };
}

macro_rules! RepoBuilder {
    () => {
        deps!();
        # [doc = " Manually construct a [`Repository`]"] # [doc = ""] # [doc = " See also [`new`], [`repo`]"] # [must_use] pub struct RepoBuilder { repo : git2 :: Repository , files : Vec < PathBuf > , }
    };
}

RepoBuilder!();