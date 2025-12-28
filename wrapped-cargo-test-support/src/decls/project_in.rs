macro_rules! deps {
    () => {
        ProjectBuilder!();
    };
}

macro_rules! project_in {
    () => {
        deps!();
        # [doc = " Generates a project layout in given directory, see [`ProjectBuilder`]"] pub fn project_in (dir : impl AsRef < Path >) -> ProjectBuilder { ProjectBuilder :: new (paths :: root () . join (dir) . join ("foo")) }
    };
}

project_in!();