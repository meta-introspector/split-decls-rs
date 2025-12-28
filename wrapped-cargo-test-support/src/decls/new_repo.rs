macro_rules! deps {
    () => {
        Repository!();
        Project!();
        ProjectBuilder!();
    };
}

macro_rules! new_repo {
    () => {
        deps!();
        # [doc = " Create a new [`Project`] with access to the [`Repository`]"] pub fn new_repo < F > (name : & str , callback : F) -> (Project , git2 :: Repository) where F : FnOnce (ProjectBuilder) -> ProjectBuilder , { let mut git_project = project () . at (name) ; git_project = callback (git_project) ; let git_project = git_project . build () ; let repo = init (& git_project . root ()) ; add (& repo) ; commit (& repo) ; (git_project , repo) }
    };
}

new_repo!();