macro_rules! deps {
    () => {
        ProjectBuilder!();
    };
}

macro_rules! project_in_home {
    () => {
        deps!();
        # [doc = " Generates a project layout inside our fake home dir, see [`ProjectBuilder`]"] pub fn project_in_home (name : impl AsRef < Path >) -> ProjectBuilder { ProjectBuilder :: new (paths :: home () . join (name)) }
    };
}

project_in_home!();