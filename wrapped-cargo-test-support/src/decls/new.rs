macro_rules! deps {
    () => {
        Project!();
        Repository!();
        ProjectBuilder!();
    };
}

macro_rules! new {
    () => {
        deps!();
        # [doc = " Create a new [`Project`] in a git [`Repository`]"] pub fn new < F > (name : & str , callback : F) -> Project where F : FnOnce (ProjectBuilder) -> ProjectBuilder , { new_repo (name , callback) . 0 }
    };
}

new!()