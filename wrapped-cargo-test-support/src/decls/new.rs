macro_rules! deps {
    () => {
        Repository!();
        ProjectBuilder!();
        Project!();
    };
}

macro_rules! new {
    () => {
        deps!();
        # [doc = " Create a new [`Project`] in a git [`Repository`]"] pub fn new < F > (name : & str , callback : F) -> Project where F : FnOnce (ProjectBuilder) -> ProjectBuilder , { new_repo (name , callback) . 0 }
    };
}

new!();